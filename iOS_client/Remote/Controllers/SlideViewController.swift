//
//  SlideViewController.swift
//  Remote
//
//  Created by Indiana Kernick on 1/2/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import Foundation

class SlideViewController: BasicViewController {
    private let lastData = Command.keyClick(Key.end)
    private let nextData = Command.keyClick(Key.rightArrow)
    private let previousData = Command.keyClick(Key.leftArrow)
    private let firstData = Command.keyClick(Key.home)
    
    @IBAction func lastPressed() {
        send(lastData)
    }
    
    @IBAction func nextPressed() {
        send(nextData)
    }
    
    @IBAction func previousPressed() {
        send(previousData)
    }
    
    @IBAction func firstPressed() {
        send(firstData)
    }
}
