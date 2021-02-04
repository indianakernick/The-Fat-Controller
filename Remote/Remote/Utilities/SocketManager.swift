//
//  SocketManager.swift
//  Remote
//
//  Created by Indiana Kernick on 29/1/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import Foundation;
import Starscream;

protocol SocketManagerDelegate: class {
    func onlineStatusChanged(online: Bool);
}

class SocketManager: WebSocketDelegate {
    private static let retryDelay = 1.0;
    private static let tickDelay = 0.05;
    private static let maxTickCount = Int(30.0 / tickDelay);
    
    private var socket: WebSocket;
    private var tickTimer: Timer?;
    private var tickCount = 0;
    private var onlineStatus = false;
    
    weak var delegate: SocketManagerDelegate?;
    
    init() {
        socket = WebSocket(url: URL(string: "ws://indi-mac.local:80/socket")!);
        socket.delegate = self;
    }
    
    func connect() {
        socket.connect();
    }
    
    func websocketDidConnect(socket: WebSocketClient) {
        updateOnlineStatus(online: true);
        tickCount = 0;
        startTicking();
    }
    
    func websocketDidDisconnect(socket: WebSocketClient, error: Error?) {
        updateOnlineStatus(online: false);
        stopTicking();
        DispatchQueue.main.asyncAfter(deadline: .now() + SocketManager.retryDelay) {
            self.connect();
        };
    }
    
    func websocketDidReceiveMessage(socket: WebSocketClient, text: String) {}
    
    func websocketDidReceiveData(socket: WebSocketClient, data: Data) {}
    
    func send(_ data: Data) {
        socket.write(data: data);
        tickCount = 0;
        if tickTimer == nil {
            startTicking();
        }
    }
    
    func send(_ data: [UInt8]) {
        send(Data(data));
    }
    
    private func startTicking() {
        tickTimer = Timer.scheduledTimer(
            timeInterval: SocketManager.tickDelay,
            target: self,
            selector: #selector(self.sendTick),
            userInfo: nil,
            repeats: true
        );
    }
    
    private func stopTicking() {
        tickTimer?.invalidate();
        tickTimer = nil;
    }
    
    @objc private func sendTick() {
        socket.write(data: Data());
        tickCount += 1;
        if tickCount > SocketManager.maxTickCount {
            stopTicking();
        }
    }
    
    private func updateOnlineStatus(online: Bool) {
        if online != onlineStatus {
            onlineStatus = online;
            delegate?.onlineStatusChanged(online: onlineStatus);
        }
    }
}
