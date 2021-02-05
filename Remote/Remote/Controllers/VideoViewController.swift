//
//  VideoViewController.swift
//  Remote
//
//  Created by Indiana Kernick on 5/2/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import Foundation;

class VideoViewController: BasicViewController {
    private let muteData = Data([CommandCode.keyClick.rawValue, Key.mute.rawValue]);
    private let volumeDownData = Data([CommandCode.keyClick.rawValue, Key.volumeDown.rawValue]);
    private let volumeUpData = Data([CommandCode.keyClick.rawValue, Key.volumeUp.rawValue]);
    private let playPauseData = Data([CommandCode.keyClick.rawValue, Key.space.rawValue]);
    private let backwardData = Data([CommandCode.keyClick.rawValue, Key.leftArrow.rawValue]);
    private let timeData = Data([CommandCode.mouseMoveRelative.rawValue, 0, 0, 0, 0]);
    private let forwardData = Data([CommandCode.keyClick.rawValue, Key.rightArrow.rawValue]);
    
    @IBAction func mutePressed() {
        send(muteData);
    }
    
    @IBAction func volumeDownPressed() {
        send(volumeDownData);
    }
    
    @IBAction func volumeUpPressed() {
        send(volumeUpData);
    }
    
    @IBAction func playPausePressed() {
        send(playPauseData);
    }
    
    @IBAction func backwardPressed() {
        send(backwardData);
    }
    
    @IBAction func timePressed() {
        send(timeData);
    }
    
    @IBAction func forwardPressed() {
        send(forwardData);
    }
}
