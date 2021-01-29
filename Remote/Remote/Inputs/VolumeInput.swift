//
//  VolumeInput.swift
//  Remote
//
//  Created by Indiana Kernick on 29/1/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import MediaPlayer

protocol VolumeInputDelegate: class {
    func volumeUpPressed();
    func volumeUpReleased();
    func volumeDownPressed();
    func volumeDownReleased();
}

class VolumeInput {
    private enum VolumeState {
        case none, up, down;
    }
    
    private var lastVolume = 0.0;
    private var volumeState = VolumeState.none;
    private var workItem: DispatchWorkItem?;
    
    // Based upon this:
    // https://stackoverflow.com/a/58661010/4093378
    
    // When one of the volume buttons is held, the volume initially changes by one
    // step. After a delay of around 300 ms, the volume rapidly changes. We can
    // choose to wait for this delay to determine whether the button is held and
    // get a continuous press. Alternatively, we can use a short delay for better
    // responsively at the custom of getting an initial click while holding.
    
    private static let longDelay = 0.33;
    private static let shortDelay = 0.1;
    
    weak var delegate: VolumeInputDelegate?;
    var continuous = false;
    
    func initialize(view: UIView) {
        lastVolume = Double(AVAudioSession.sharedInstance().outputVolume);
        volumeState = VolumeState.none;
        
        let volumeChangedSystemName = NSNotification.Name(rawValue: "AVSystemController_SystemVolumeDidChangeNotification");
        NotificationCenter.default.addObserver(self, selector: #selector(volumeChanged), name: volumeChangedSystemName, object: nil);
        let volumeView = MPVolumeView(frame: CGRect(x: -CGFloat.greatestFiniteMagnitude, y: 0, width: 0, height: 0));
        view.addSubview(volumeView);
    }
    
    private func volumeIncreased() -> Bool {
        let short: Bool;
        switch volumeState {
        case .none:
            volumeState = .up;
            delegate?.volumeUpPressed();
            short = false;
        case .up:
            short = true;
        case .down:
            volumeState = .up;
            delegate?.volumeDownReleased();
            delegate?.volumeUpPressed();
            short = false;
        }
        workItem = DispatchWorkItem {
            self.volumeState = .none;
            self.delegate?.volumeUpReleased();
            self.workItem = nil;
        };
        return short;
    }
    
    private func volumeDecreased() -> Bool {
        let short: Bool;
        switch volumeState {
        case .none:
            volumeState = .down;
            delegate?.volumeDownPressed();
            short = false;
        case .up:
            volumeState = .down;
            delegate?.volumeUpReleased();
            delegate?.volumeDownPressed();
            short = false;
        case .down:
            short = true;
        }
        workItem = DispatchWorkItem {
            self.volumeState = .none;
            self.delegate?.volumeDownReleased();
            self.workItem = nil;
        };
        return short;
    }

    @objc private func volumeChanged(notification: NSNotification) {
        guard
            let userInfo = notification.userInfo,
            let reason = userInfo["AVSystemController_AudioVolumeChangeReasonNotificationParameter"] as? String,
            reason == "ExplicitVolumeChange",
            let newVolume = userInfo["AVSystemController_AudioVolumeNotificationParameter"] as? Double
            else { return }

        if let workItem = workItem {
            workItem.cancel();
        }

        let short: Bool;
        if newVolume > lastVolume || newVolume == 1.0 {
            short = volumeIncreased();
        } else if newVolume < lastVolume || newVolume == 0.0 {
            short = volumeDecreased();
        } else {
            return;
        }

        let delay = short || !continuous ? VolumeInput.shortDelay : VolumeInput.longDelay;
        DispatchQueue.main.asyncAfter(deadline: .now() + delay, execute: workItem!);
        lastVolume = newVolume;
    }
}
