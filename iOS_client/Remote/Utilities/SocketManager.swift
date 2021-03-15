//
//  SocketManager.swift
//  Remote
//
//  Created by Indiana Kernick on 29/1/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import Network
import Foundation

protocol SocketManagerDelegate: class {
    func onlineStatusChanged(online: Bool)
}

class SocketManager : ConnectionDelegate {
    private static let retryDelay = 1.0
    private static let tickDelay = 0.05
    private static let maxTickCount = Int(30.0 / tickDelay)
    private static let tickData = Data([255])
    
    private var connection = Connection()
    private var onlineStatus = false
    private var tickTimer: Timer?
    private var tickCount = 0
    
    weak var delegate: SocketManagerDelegate?
    
    init() {
        connection.delegate = self
    }
    
    func connect(host: String, port: UInt16) {
        updateOnlineStatus(online: false)
        stopTicking()
        // We get an empty host string when there is no host stored in
        // UserDefaults. NW doesn't like empty host strings.
        if !host.isEmpty {
            connection.connect(host: host, port: port)
        }
    }
    
    func send(_ data: Data) {
        connection.send(data)
        tickCount = 0
        if tickTimer == nil {
            startTicking()
        }
    }
    
    func send(_ data: [UInt8]) {
        send(Data(data))
    }
    
    func connectionGained() {
        DispatchQueue.main.async {
            self.updateOnlineStatus(online: true)
            self.tickCount = 0
            self.startTicking()
        }
    }
    
    func connectionLost() {
        DispatchQueue.main.async {
            self.updateOnlineStatus(online: false)
            self.stopTicking()
        }
        DispatchQueue.main.asyncAfter(deadline: .now() + SocketManager.retryDelay) {
            // When we reconnect, the old connection will close and the new
            // connection open. Then we will reach this point after successfully
            // connecting and try to reconnect even though we're already
            // connected.
            if self.connection.isDisconnected() {
                self.connection.connect()
            }
        }
    }
    
    private func updateOnlineStatus(online: Bool) {
        if online != onlineStatus {
            onlineStatus = online
            delegate?.onlineStatusChanged(online: onlineStatus)
        }
    }
    
    private func startTicking() {
        tickTimer = Timer.scheduledTimer(
            timeInterval: SocketManager.tickDelay,
            target: self,
            selector: #selector(self.sendTick),
            userInfo: nil,
            repeats: true
        )
    }
    
    private func stopTicking() {
        tickTimer?.invalidate()
        tickTimer = nil
    }
    
    @objc private func sendTick() {
        connection.send(SocketManager.tickData)
        tickCount += 1
        if tickCount > SocketManager.maxTickCount {
            stopTicking()
        }
    }
}
