//
//  TrackpadViewController.swift
//  Remote
//
//  Created by Indiana Kernick on 1/2/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import Foundation;

class TrackpadViewController: BasicViewController, TrackpadInputDelegate {
    @IBOutlet weak var trackpad: TrackpadInput!;
    
    override func viewDidLoad() {
        super.viewDidLoad();
        trackpad.delegate = self;
    }
    
    func mouseClick() {
        send(Data([CommandCode.mouseClick.rawValue, MouseButton.left.rawValue]));
    }
    
    func mouseDoubleClick() {
        send(Data([CommandCode.mouseNthClick.rawValue, MouseButton.left.rawValue, 2]));
    }
    
    func mouseTripleClick() {
        send(Data([CommandCode.mouseNthClick.rawValue, MouseButton.left.rawValue, 3]));
    }
    
    func mouseRightClick() {
        send(Data([CommandCode.mouseClick.rawValue, MouseButton.right.rawValue]));
    }
    
    func mouseMove(dx: Int32, dy: Int32) {
        
    }
    
    func mouseScroll(dx: Int32, dy: Int32) {
        
    }
    
    func mouseDown() {
        
    }
    
    func mouseUp() {
        
    }
}
