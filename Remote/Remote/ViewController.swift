//
//  ViewController.swift
//  Remote
//
//  Created by Indiana Kernick on 28/1/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import UIKit

// Maybe the client and server should maintain a random number generator.
// Client sends a random number with every message.
// Client number must match server number. Otherwise server ignores message.
// Probably less latency than full encryption. It's not like I'm going to be
// typing passwords with this.

// Could maybe use TCP instead of websockets. That would require dropping the
// web client completely and doing everything in the app.

// MoveTranslator
//   translate the input into socket messages

// Make it possible to configure the controller from within the app

// Don't use the storyboard. Create views from Swift.
// Make socket messages configurable on an object.

class ViewController: UIViewController, VolumeInputDelegate, MoveInputDelegate, LookInputDelegate, SocketManagerDelegate, ButtonInputDelegate {
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
        lookInput.slowScale = 1.9;
        lookInput.middleScale = 2.4;
        lookInput.fastScale = 4.2;
        lookInput.slowVelocity = 40.0;
        lookInput.fastVelocity = 200.0;
        lookInput.delegate = self;
        
        let buttonA = ViewController.addTopLeftButton(parent: view.subviews[0]);
        buttonA.color = Colors.gray500;
        buttonA.button = .a;
        buttonA.delegate = self;
        
        let buttonB = ViewController.addBottomLeftButton(parent: view.subviews[0]);
        buttonB.color = Colors.gray500;
        buttonB.button = .b;
        buttonB.delegate = self;
        
        let buttonC = ViewController.addBottomRightButton(parent: view.subviews[1]);
        buttonC.color = Colors.gray500;
        buttonC.button = .c;
        buttonC.delegate = self;
        
        let buttonD = ViewController.addTopRightButton(parent: view.subviews[1]);
        buttonD.color = Colors.gray500;
        buttonD.button = .d;
        buttonD.delegate = self;
    }
    
    private static func addButton(parent: UIView) -> ButtonInput {
        let button = ButtonInput();
        button.translatesAutoresizingMaskIntoConstraints = false;
        parent.addSubview(button);
        parent.addConstraint(button.widthAnchor.constraint(equalToConstant: 70.0));
        parent.addConstraint(button.heightAnchor.constraint(equalToConstant: 70.0));
        return button;
    }
    
    private static func addTopLeftButton(parent: UIView) -> ButtonInput {
        let button = addButton(parent: parent);
        parent.addConstraint(button.topAnchor.constraint(equalTo: parent.topAnchor));
        parent.addConstraint(button.leftAnchor.constraint(equalTo: parent.leftAnchor));
        return button;
    }
    
    private static func addTopRightButton(parent: UIView) -> ButtonInput {
        let button = addButton(parent: parent);
        parent.addConstraint(button.topAnchor.constraint(equalTo: parent.topAnchor));
        parent.addConstraint(button.rightAnchor.constraint(equalTo: parent.rightAnchor));
        return button;
    }
    
    private static func addBottomRightButton(parent: UIView) -> ButtonInput {
        let button = addButton(parent: parent);
        parent.addConstraint(button.bottomAnchor.constraint(equalTo: parent.bottomAnchor));
        parent.addConstraint(button.rightAnchor.constraint(equalTo: parent.rightAnchor));
        return button;
    }
    
    private static func addBottomLeftButton(parent: UIView) -> ButtonInput {
        let button = addButton(parent: parent);
        parent.addConstraint(button.bottomAnchor.constraint(equalTo: parent.bottomAnchor));
        parent.addConstraint(button.leftAnchor.constraint(equalTo: parent.leftAnchor));
        return button;
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
    
    func buttonPressed(button: Button) {
        switch button {
        case .a:
            socket.send([CommandCode.keyDown.rawValue, Key.e.rawValue]);
        case .b:
            socket.send([CommandCode.keyDown.rawValue, Key.q.rawValue]);
        case .c:
            socket.send([CommandCode.keyDown.rawValue, Key.space.rawValue]);
        case .d:
            socket.send([CommandCode.keyDown.rawValue, Key.escape.rawValue]);
        default:
            break;
        }
    }
    
    func buttonReleased(button: Button) {
        switch button {
        case .a:
            socket.send([CommandCode.keyUp.rawValue, Key.e.rawValue]);
        case .b:
            socket.send([CommandCode.keyUp.rawValue, Key.q.rawValue]);
        case .c:
            socket.send([CommandCode.keyUp.rawValue, Key.space.rawValue]);
        case .d:
            socket.send([CommandCode.keyUp.rawValue, Key.escape.rawValue]);
        default:
            break;
        }
    }
    
    func onlineStatusChanged(online: Bool) {
        for view in view.subviews {
            view.isHidden = !online;
        }
    }
}
