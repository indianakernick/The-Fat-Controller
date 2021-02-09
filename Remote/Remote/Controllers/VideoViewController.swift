//
//  VideoViewController.swift
//  Remote
//
//  Created by Indiana Kernick on 5/2/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import Foundation

class VideoViewController: BasicViewController {
    private let muteData = Command.keyClick(Key.mute)
    private let volumeDownData = Command.keyClick(Key.volumeDown)
    private let volumeUpData = Command.keyClick(Key.volumeUp)
    private let playPauseData = Command.keyClick(Key.space)
    private let backwardData = Command.keyClick(Key.leftArrow)
    private let timeData = Command.mouseMoveRel()
    private let forwardData = Command.keyClick(Key.rightArrow)
    
    @IBAction func mutePressed() {
        send(muteData)
    }
    
    @IBAction func volumeDownPressed() {
        send(volumeDownData)
    }
    
    @IBAction func volumeUpPressed() {
        send(volumeUpData)
    }
    
    @IBAction func playPausePressed() {
        send(playPauseData)
    }
    
    @IBAction func backwardPressed() {
        send(backwardData)
    }
    
    @IBAction func timePressed() {
        send(timeData)
    }
    
    @IBAction func forwardPressed() {
        send(forwardData)
    }
}
