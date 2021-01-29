//
//  ViewController.swift
//  Remote
//
//  Created by Indiana Kernick on 28/1/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import UIKit
import Starscream

// Maybe the client and server should maintain a random number generator.
// Client sends a random number with every message.
// Client number must match server number. Otherwise server ignores message.
// Probably less latency than full encryption. It's not like I'm going to be
// typing passwords with this.

// Could maybe use TCP instead of websockets. That would require dropping the
// web client completely and doing everything in the app.

class ViewController: UIViewController, WebSocketDelegate, VolumeInputDelegate {
    func websocketDidConnect(socket: WebSocketClient) {}
    func websocketDidDisconnect(socket: WebSocketClient, error: Error?) {}
    func websocketDidReceiveMessage(socket: WebSocketClient, text: String) {}
    func websocketDidReceiveData(socket: WebSocketClient, data: Data) {}

    private var upLabel = UILabel(frame: CGRect(x: 10.0, y: 10.0, width: 100, height: 20));
    private var downLabel = UILabel(frame: CGRect(x: 10.0, y: 30.0, width: 100, height: 20));
    
    private var socket: WebSocket!;
    private var volumeInput = VolumeInput();

    override func viewDidLoad() {
        super.viewDidLoad()

        view.subviews[0].addSubview(upLabel);
        view.subviews[0].addSubview(downLabel);
        
        volumeInput.delegate = self;
        volumeInput.continuous = true;
        volumeInput.initialize(view: view);
        
        socket = WebSocket(url: URL(string: "ws://indi-mac.local:80/socket")!);
        socket.delegate = self;
        socket.connect();
    }

    internal func volumeUpPressed() {
        upLabel.text = "Up";
        socket.write(data: Data([CommandCode.mouseDown.rawValue, MouseButton.right.rawValue]));
    }
  
    internal func volumeUpReleased() {
        upLabel.text = "";
        socket.write(data: Data([CommandCode.mouseUp.rawValue, MouseButton.right.rawValue]));
    }
  
    internal func volumeDownPressed() {
        downLabel.text = "Down";
        socket.write(data: Data([CommandCode.mouseDown.rawValue, MouseButton.left.rawValue]));
    }
  
    internal func volumeDownReleased() {
        downLabel.text = "";
        socket.write(data: Data([CommandCode.mouseUp.rawValue, MouseButton.left.rawValue]));
    }
}
