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

// MoveTranslator
//   translate the input into socket messages

class ViewController: UIViewController, VolumeInputDelegate, MoveInputDelegate, LookInputDelegate, SocketManagerDelegate {
    private var upLabel = UILabel(frame: CGRect(x: 10.0, y: 10.0, width: 100, height: 20));
    private var downLabel = UILabel(frame: CGRect(x: 10.0, y: 30.0, width: 100, height: 20));
    
    private var socket = SocketManager();
    private var volumeInput = VolumeInput();

    override func viewDidLoad() {
        super.viewDidLoad();

        view.subviews[0].addSubview(upLabel);
        view.subviews[0].addSubview(downLabel);
        
        socket.delegate = self;
        socket.connect();
        
        volumeInput.delegate = self;
        volumeInput.continuous = false;
        volumeInput.initialize(view: view);
        
        let moveInput = view.subviews[0] as! MoveView;
        moveInput.origin = .relative;
        moveInput.stationaryThreshold = 20.0;
        moveInput.delegate = self;
        
        let lookInput = view.subviews[1] as! LookView;
        lookInput.scale = 2.5;
        lookInput.delegate = self;
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
    
    private static func setInt16(buf: inout [UInt8], index: Int, value: Int16) {
        buf[index] = UInt8((value >> 8) & 0xFF);
        buf[index + 1] = UInt8(value & 0xFF);
    }
    
    func lookDirectionChanged(dx: Int32, dy: Int32) {
        var buffer = [UInt8](repeating: 0, count: 5);
        buffer[0] = CommandCode.mouseMoveRelative.rawValue;
        ViewController.setInt16(buf: &buffer, index: 1, value: Int16(dx));
        ViewController.setInt16(buf: &buffer, index: 3, value: Int16(dy));
        socket.send(buffer);
    }
    
    func moveDirectionChanged(old: MoveDirection, new: MoveDirection) {
        let keys = [Key.w.rawValue, Key.d.rawValue, Key.s.rawValue, Key.a.rawValue];
        var buffer = [UInt8]();
        for d in 0...4 {
            let bit = UInt8(1 << d);
            if new.rawValue & bit != 0 && old.rawValue & bit == 0 {
                buffer.append(CommandCode.keyDown.rawValue);
                buffer.append(keys[d]);
            } else if new.rawValue & bit == 0 && old.rawValue & bit != 0 {
                buffer.append(CommandCode.keyUp.rawValue);
                buffer.append(keys[d]);
            }
        }
        socket.send(buffer);
    }
    
    func onlineStatusChanged(online: Bool) {
        for view in view.subviews {
            view.isHidden = !online;
        }
    }
}
