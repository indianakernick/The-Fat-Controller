//
//  Haptic.swift
//  Remote
//
//  Created by Indiana Kernick on 28/1/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import AudioToolbox

// https://stackoverflow.com/a/42990919/4093378

enum Haptic {
    static func buzz() {
        AudioServicesPlaySystemSound(kSystemSoundID_Vibrate);
    }
    
    static func weakTap() {
        AudioServicesPlaySystemSound(1519);
    }
    
    static func strongTap() {
        AudioServicesPlaySystemSound(1520);
    }
    
    static func tripleWeakTap() {
        AudioServicesPlaySystemSound(1521);
    }
}
