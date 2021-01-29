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

class ViewController: UIViewController, VolumeInputDelegate, SocketManagerDelegate {
    private var upLabel = UILabel(frame: CGRect(x: 10.0, y: 10.0, width: 100, height: 20));
    private var downLabel = UILabel(frame: CGRect(x: 10.0, y: 30.0, width: 100, height: 20));
    
    private var socket = SocketManager();
    private var volumeInput = VolumeInput();

    override func viewDidLoad() {
        super.viewDidLoad()

        view.subviews[0].addSubview(upLabel);
        view.subviews[0].addSubview(downLabel);
        
        socket.delegate = self;
        socket.connect();
        
        volumeInput.delegate = self;
        volumeInput.continuous = true;
        volumeInput.initialize(view: view);
    }

    func volumeUpPressed() {
        upLabel.text = "Up";
        socket.send([CommandCode.mouseDown.rawValue, MouseButton.right.rawValue]);
    }
  
    func volumeUpReleased() {
        upLabel.text = "";
        socket.send([CommandCode.mouseUp.rawValue, MouseButton.right.rawValue]);
    }
  
    func volumeDownPressed() {
        downLabel.text = "Down";
        socket.send([CommandCode.mouseDown.rawValue, MouseButton.left.rawValue]);
    }
  
    func volumeDownReleased() {
        downLabel.text = "";
        socket.send([CommandCode.mouseUp.rawValue, MouseButton.left.rawValue]);
    }
    
    func onlineStatusChanged(online: Bool) {
        view.isHidden = !online;
    }
}
