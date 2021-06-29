//
//  VideoVC.swift
//  Remote
//
//  Created by Indiana Kernick on 5/2/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import Foundation

class VideoVC: BasicVC {
    private let muteData = CommandData.keyClick(Key.mute)
    private let volumeDownData = CommandData.keyClick(Key.volumeDown)
    private let volumeUpData = CommandData.keyClick(Key.volumeUp)
    private let playPauseData = CommandData.keyClick(Key.space)
    private let backwardData = CommandData.keyClick(Key.leftArrow)
    private let timeData = CommandData.mouseMoveRel()
    private let forwardData = CommandData.keyClick(Key.rightArrow)
    
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
