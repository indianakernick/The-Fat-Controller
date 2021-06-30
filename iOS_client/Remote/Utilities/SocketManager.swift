//
//  SocketManager.swift
//  Remote
//
//  Created by Indiana Kernick on 29/1/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import Foundation
import Starscream
import CryptoKit

protocol SocketManagerDelegate: AnyObject {
    func onlineStatusChanged(online: Bool)
}

class SocketManager: WebSocketDelegate {
    private static let retryDelay = 1.0
    private static let tickDelay = 0.05
    private static let maxTickCount = Int(30.0 / tickDelay)
    private static let emptyData = Data()
    private static let encryptionEnabledData = Data([1])
    private static let encryptionDisabledData = Data([0])
    
    private var socket: WebSocket!
    private var tickTimer: Timer?
    private var tickCount = 0
    private var retrying = false
    private var onlineStatus = false
    private var host = ""
    private var dummyMode = false
    private var lowLatencyMode = true
    private var secureMode = true
    private var secureKey: SymmetricKey? = SymmetricKey(data: Data([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]))
    
    private func startTicking() {
        if lowLatencyMode {
            tickTimer = Timer.scheduledTimer(
                timeInterval: SocketManager.tickDelay,
                target: self,
                selector: #selector(self.sendTick),
                userInfo: nil,
                repeats: true
            )
        }
    }
    
    private func stopTicking() {
        tickTimer?.invalidate()
        tickTimer = nil
    }
    
    @objc private func sendTick() {
        if dummyMode || tickTimer == nil { return }
        socket.write(data: SocketManager.emptyData)
        tickCount += 1
        if tickCount > SocketManager.maxTickCount {
            stopTicking()
        }
    }
    
    private func updateOnlineStatus(online: Bool) {
        if online != onlineStatus {
            onlineStatus = online
            delegate?.onlineStatusChanged(online: onlineStatus)
        }
    }
    
    // --- SocketManager --- //

    weak var delegate: SocketManagerDelegate?
    
    // Connecting
    
    func connectTo(host: String) {
        self.host = host
        stopTicking()
        updateOnlineStatus(online: false)
        
        if host == "dummy" {
            dummyMode = true
            updateOnlineStatus(online: true)
        } else {
            dummyMode = false
            if let url = URL(string: "ws://" + host + ":80") {
                socket = WebSocket(url: url)
                socket.delegate = self
                socket.connect()
            }
        }
    }

    func reconnect() {
        if !dummyMode {
            socket.connect()
        }
    }
    
    // Sending
    
    func send(_ data: Data) {
        if dummyMode { return }
        
        if secureMode {
            if secureKey == nil { return }
            if data.count == 0 {
                socket.write(data: data)
            } else {
                let nonce = AES.GCM.Nonce.init()
                do {
                    let sealedBox = try AES.GCM.seal(data, using: secureKey!, nonce: nonce)
                    socket.write(data: sealedBox.combined!)
                } catch {
                    print(error)
                    return
                }
            }
        } else {
            socket.write(data: data)
        }
        
        tickCount = 0
        if tickTimer == nil {
            startTicking()
        }
    }
    
    func send(_ data: [UInt8]) {
        send(Data(data))
    }
    
    // Introspection
    
    func getOnlineStatus() -> Bool {
        onlineStatus
    }
    
    func getOnlineHost() -> String? {
        onlineStatus ? host : nil
    }
    
    // Configuration
    
    func setLowLatencyMode(enabled: Bool) {
        lowLatencyMode = enabled
        if lowLatencyMode {
            startTicking()
        } else {
            stopTicking()
        }
    }
    
    func setSecureMode(enabled: Bool) {
        secureMode = enabled
        secureKey = nil
        // disconnect
        // reconnect and specify whether we want encryption or not
        // if we do, then the server will generate a key and create a QR code
        // the user will scan the QR code
        // until the user does that, nothing will happen
        // upon successfully scanning the QR code, setSecureKey will be called
        // and we may begin
    }
    
    func setSecureKey(key: SymmetricKey) {
        secureKey = key
    }
    
    // --- WebSocketDelegate --- //
    
    func websocketDidConnect(socket: WebSocketClient) {
        /*socket.write(data:
            secureMode
            ? SocketManager.encryptionEnabledData
            : SocketManager.encryptionDisabledData
        )*/
        socket.write(data: SocketManager.encryptionEnabledData)
        updateOnlineStatus(online: true)
        tickCount = 0
        startTicking()
    }

    func websocketDidDisconnect(socket: WebSocketClient, error: Error?) {
        stopTicking()
        updateOnlineStatus(online: false)
        if !retrying {
            retrying = true
            DispatchQueue.main.asyncAfter(deadline: .now() + SocketManager.retryDelay) {
                self.retrying = false
                self.reconnect()
            }
        }
    }

    func websocketDidReceiveMessage(socket: WebSocketClient, text: String) {}

    func websocketDidReceiveData(socket: WebSocketClient, data: Data) {}
}
