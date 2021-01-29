//
//  SocketManager.swift
//  Remote
//
//  Created by Indiana Kernick on 29/1/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import Starscream

protocol SocketManagerDelegate: class {
    func onlineStatusChanged(online: Bool);
}

class SocketManager: WebSocketDelegate {
    private static let retryDelay = 1.0;
    
    private var socket: WebSocket!;
    
    weak var delegate: SocketManagerDelegate?;
    
    init() {
        socket = WebSocket(url: URL(string: "ws://indi-mac.local:80/socket")!);
        socket.delegate = self;
    }
    
    func connect() {
        socket.connect();
    }
    
    func websocketDidConnect(socket: WebSocketClient) {
        delegate?.onlineStatusChanged(online: true);
    }
    
    func websocketDidDisconnect(socket: WebSocketClient, error: Error?) {
        delegate?.onlineStatusChanged(online: false);
        DispatchQueue.main.asyncAfter(deadline: .now() + SocketManager.retryDelay) {
            self.connect();
        }
    }
    
    func websocketDidReceiveMessage(socket: WebSocketClient, text: String) {}
    func websocketDidReceiveData(socket: WebSocketClient, data: Data) {}
    
    func send(_ data: [UInt8]) {
        socket.write(data: Data(data));
    }
}
