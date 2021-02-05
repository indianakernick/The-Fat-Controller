//
//  SlideViewController.swift
//  Remote
//
//  Created by Indiana Kernick on 1/2/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import Foundation

class SlideViewController: BasicViewController {
    private let lastData = Data([CommandCode.keyClick.rawValue, Key.end.rawValue])
    private let nextData = Data([CommandCode.keyClick.rawValue, Key.rightArrow.rawValue])
    private let previousData = Data([CommandCode.keyClick.rawValue, Key.leftArrow.rawValue])
    private let firstData = Data([CommandCode.keyClick.rawValue, Key.home.rawValue])
    
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
