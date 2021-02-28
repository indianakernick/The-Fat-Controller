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

class SocketManager {
    private static let port: UInt16 = 2048;
    private static let retryDelay = 1.0
    private static let tickDelay = 0.05
    private static let maxTickCount = Int(30.0 / tickDelay)
    private static let tickData = Data([255])
    
    private var connection: NWConnection!
    private let queue = DispatchQueue(label: "Queue")
    private var onlineStatus = false
    private var tickTimer: Timer?
    private var tickCount = 0
    
    weak var delegate: SocketManagerDelegate?
    
    func connectTo(host: String) {
        updateOnlineStatus(online: false)
        stopTicking()
        connectTo(host: NWEndpoint.Host(host))
    }
    
    func send(_ data: Data) {
        sendToConnection(data)
        tickCount = 0
        if tickTimer == nil {
            startTicking()
        }
    }
    
    func send(_ data: [UInt8]) {
        send(Data(data))
    }
    
    private func connectTo(host: NWEndpoint.Host) {
        connection = NWConnection(
            host: host,
            port: NWEndpoint.Port(integerLiteral: SocketManager.port),
            using: .tcp
        )
        connection.stateUpdateHandler = stateChanged
        receive()
        connection.start(queue: queue)
    }
    
    private func connected() {
        DispatchQueue.main.async {
            self.updateOnlineStatus(online: true)
            self.tickCount = 0
            self.startTicking()
        }
    }
    
    private func disconnected() {
        DispatchQueue.main.async {
            self.updateOnlineStatus(online: false)
            self.stopTicking()
        }
        DispatchQueue.main.asyncAfter(deadline: .now() + SocketManager.retryDelay) {
            if case let NWEndpoint.hostPort(host, _) = self.connection.endpoint {
                self.connectTo(host: host)
            }
        }
    }
    
    private func stateChanged(to: NWConnection.State) {
        switch to {
        case .setup:
            break
        case .preparing:
            break
        case .ready:
            connected()
        case .waiting(_):
            fallthrough
        case .failed(_):
            disconnected()
        case .cancelled:
            fallthrough
        @unknown default:
            break
        }
    }
    
    private func updateOnlineStatus(online: Bool) {
        if online != self.onlineStatus {
            self.onlineStatus = online
            self.delegate?.onlineStatusChanged(online: self.onlineStatus)
        }
    }
    
    private func receive() {
        connection.receive(minimumIncompleteLength: 1, maximumLength: 65536) {
            (_, _, isComplete, error) in
            if isComplete || error != nil {
                self.disconnected()
            } else {
                self.receive()
            }
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
        sendToConnection(SocketManager.tickData)
        tickCount += 1
        if tickCount > SocketManager.maxTickCount {
            stopTicking()
        }
    }
    
    private func sendToConnection(_ data: Data) {
        connection.send(content: data, completion: .contentProcessed({ error in
            if let error = error {
                print(error)
            }
        }))
    }
}
