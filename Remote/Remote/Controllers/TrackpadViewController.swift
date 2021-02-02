//
//  TrackpadViewController.swift
//  Remote
//
//  Created by Indiana Kernick on 1/2/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import Foundation;

fileprivate func setInt16(buf: inout Data, index: Int, value: Int16) {
    buf[index] = UInt8((value >> 8) & 0xFF);
    buf[index + 1] = UInt8(value & 0xFF);
}

class TrackpadViewController: BasicViewController, TrackpadInputDelegate {
    @IBOutlet weak var trackpad: TrackpadInput!;
    
    private var clickData = Data([CommandCode.mouseClick.rawValue, MouseButton.left.rawValue]);
    private var doubleClickData = Data([CommandCode.mouseNthClick.rawValue, MouseButton.left.rawValue, 2]);
    private var tripleClickData = Data([CommandCode.mouseNthClick.rawValue, MouseButton.left.rawValue, 3]);
    private var rightClickData = Data([CommandCode.mouseClick.rawValue, MouseButton.right.rawValue]);
    private var moveData = Data([CommandCode.mouseMoveRelative.rawValue, 0, 0, 0, 0]);
    private var scrollXData = Data([CommandCode.mouseScrollX.rawValue, 0, 0]);
    private var scrollYData = Data([CommandCode.mouseScrollY.rawValue, 0, 0]);
    private var scrollXYData = Data([CommandCode.mouseScrollX.rawValue, 0, 0, CommandCode.mouseScrollY.rawValue, 0, 0]);
    private var downData = Data([CommandCode.mouseDown.rawValue, MouseButton.left.rawValue]);
    private var upData = Data([CommandCode.mouseUp.rawValue, MouseButton.left.rawValue]);
    private var spaceLeftData = Data([CommandCode.keyClickFlags.rawValue, Key.leftArrow.rawValue, Flags.control.rawValue]);
    private var spaceRightData = Data([CommandCode.keyClickFlags.rawValue, Key.rightArrow.rawValue, Flags.control.rawValue]);
    
    override func viewDidLoad() {
        super.viewDidLoad();
        trackpad.slowMoveScale = 1.0;
        trackpad.middleMoveScale = 2.4;
        trackpad.fastMoveScale = 3.6;
        trackpad.slowSpeed = 50.0;
        trackpad.fastSpeed = 300.0;
        trackpad.scrollScale = 1.8;
        trackpad.delegate = self;
    }
    
    func mouseClick() {
        send(clickData);
    }
    
    func mouseDoubleClick() {
        send(doubleClickData);
    }
    
    func mouseTripleClick() {
        send(tripleClickData);
    }
    
    func mouseRightClick() {
        send(rightClickData);
    }
    
    func mouseMove(dx: Int32, dy: Int32) {
        if dx != 0 || dy != 0 {
            setInt16(buf: &moveData, index: 1, value: Int16(dx));
            setInt16(buf: &moveData, index: 3, value: Int16(dy));
            send(moveData);
        }
    }
    
    func mouseScroll(dx: Int32, dy: Int32) {
        if dx != 0 && dy != 0 {
            setInt16(buf: &scrollXYData, index: 1, value: Int16(dx));
            setInt16(buf: &scrollXYData, index: 4, value: Int16(dy));
            send(scrollXYData);
        } else if dx != 0 {
            setInt16(buf: &scrollXData, index: 1, value: Int16(dx));
            send(scrollXData);
        } else if dy != 0 {
            setInt16(buf: &scrollYData, index: 1, value: Int16(dy));
            send(scrollYData);
        }
    }
    
    func mouseDown() {
        send(downData);
    }
    
    func mouseUp() {
        send(upData);
    }
    
    func spaceLeft() {
        send(spaceLeftData);
    }
    
    func spaceRight() {
        send(spaceRightData);
    }
}
