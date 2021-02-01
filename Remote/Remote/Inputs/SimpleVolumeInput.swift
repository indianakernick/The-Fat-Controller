//
//  SimpleVolumeInput.swift
//  Remote
//
//  Created by Indiana Kernick on 1/2/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import UIKit;
import MediaPlayer;

class SimpleVolumeInput: MPVolumeView {
    private var lastVolume = 0.0;
    
    var upPressed = {};
    var downPressed = {};
    
    override func layoutSubviews() {
        super.layoutSubviews();
        frame.origin = CGPoint(x: -1000, y: 0);
        frame.size = CGSize(width: 100, height: 100);
        
        lastVolume = Double(AVAudioSession.sharedInstance().outputVolume);
        NotificationCenter.default.addObserver(
            self,
            selector: #selector(volumeChanged),
            name: NSNotification.Name(rawValue: "AVSystemController_SystemVolumeDidChangeNotification"),
            object: nil
        );
    }
    
    @objc private func volumeChanged(notification: NSNotification) {
        guard
            let userInfo = notification.userInfo,
            let reason = userInfo["AVSystemController_AudioVolumeChangeReasonNotificationParameter"] as? String,
            reason == "ExplicitVolumeChange",
            let newVolume = userInfo["AVSystemController_AudioVolumeNotificationParameter"] as? Double
            else { return }

        if newVolume > lastVolume || newVolume == 1.0 {
            upPressed();
        } else if newVolume < lastVolume || newVolume == 0.0 {
            downPressed();
        } else {
            return;
        }

        lastVolume = newVolume;
    }
}
