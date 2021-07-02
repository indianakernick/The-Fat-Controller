//
//  SlideVC.swift
//  Remote
//
//  Created by Indiana Kernick on 1/2/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import Foundation

class SlideVC: BasicVC {
    private let lastData = CommandData.keyClick(Key.end)
    private let nextData = CommandData.keyClick(Key.rightArrow)
    private let previousData = CommandData.keyClick(Key.leftArrow)
    private let firstData = CommandData.keyClick(Key.home)
    
    // --- Interface Builder --- //
    
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
