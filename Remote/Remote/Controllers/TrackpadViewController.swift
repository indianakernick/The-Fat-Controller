//
//  TrackpadViewController.swift
//  Remote
//
//  Created by Indiana Kernick on 1/2/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import Foundation

class TrackpadViewController: BasicViewController, TrackpadInputDelegate {
    @IBOutlet weak var trackpad: TrackpadInput!
    
    private var clickData = Command.mouseClick(MouseButton.left)
    private var rightClickData = Command.mouseClick(MouseButton.right)
    private var moveData = Command.mouseMoveRel()
    private var scrollData = Command.mouseScroll()
    private var downData = Command.mouseDown(MouseButton.left)
    private var upData = Command.mouseUp(MouseButton.left)
    private var spaceLeftData = Command.keyClick(Key.leftArrow, with: Key.control)
    private var spaceRightData = Command.keyClick(Key.rightArrow, with: Key.control)
    
    override func viewDidLoad() {
        super.viewDidLoad()
        trackpad.slowMoveScale = 1.0
        trackpad.middleMoveScale = 2.4
        trackpad.fastMoveScale = 3.6
        trackpad.slowSpeed = 50.0
        trackpad.fastSpeed = 300.0
        trackpad.scrollScale = 1.8
        trackpad.delegate = self
    }
    
    func mouseClick() {
        send(clickData)
    }
    
    func mouseDoubleClick() {
        send(clickData)
    }
    
    func mouseTripleClick() {
        send(clickData)
    }
    
    func mouseRightClick() {
        send(rightClickData)
    }
    
    func mouseMove(dx: Int32, dy: Int32) {
        if dx != 0 || dy != 0 {
            Command.setMouseParams(&moveData, x: Int16(dx), y: Int16(dy));
            send(moveData)
        }
    }
    
    func mouseScroll(dx: Int32, dy: Int32) {
        if dx != 0 || dy != 0 {
            Command.setMouseParams(&scrollData, x: Int16(-dx), y: Int16(-dy));
            send(scrollData)
        }
    }
    
    func mouseDown() {
        send(downData)
    }
    
    func mouseUp() {
        send(upData)
    }
    
    func spaceLeft() {
        send(spaceLeftData)
    }
    
    func spaceRight() {
        send(spaceRightData)
    }
}
