//
//  ViewController.swift
//  Remote
//
//  Created by Indiana Kernick on 28/1/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import UIKit
import MediaPlayer

class ViewController: UIViewController {
  private var upLabel = UILabel(frame: CGRect(x: 10.0, y: 10.0, width: 100, height: 20));
  private var downLabel = UILabel(frame: CGRect(x: 10.0, y: 30.0, width: 100, height: 20));
  
  private enum VolumeState {
    case None, Up, Down
  }
  
  private var lastVolume = Double();
  private var volumeState = VolumeState.None;
  private var workItem: DispatchWorkItem? = nil;
  
  // Based upon this:
  // https://stackoverflow.com/a/58661010/4093378
  
  // When one of the volume buttons is held, the volume initially changes by one
  // step. After a delay of around 300 ms, the volume rapidly changes. We can
  // choose to wait for this delay to determine whether the button is held and
  // get a continuous press. Alternatively, we can use a short delay for better
  // responsively at the custom of getting an initial click while holding.
  
  private static let longDelay = 3.2;
  private static let shortDelay = 0.1;
  private static let delay = shortDelay;

  override func viewDidLoad() {
    super.viewDidLoad()
    let volumeChangedSystemName = NSNotification.Name(rawValue: "AVSystemController_SystemVolumeDidChangeNotification");
    NotificationCenter.default.addObserver(self, selector: #selector(volumeChanged), name: volumeChangedSystemName, object: nil);
    let volumeView = MPVolumeView(frame: CGRect(x: -CGFloat.greatestFiniteMagnitude, y: 0, width: 0, height: 0));
    view.addSubview(volumeView);
    lastVolume = Double(AVAudioSession.sharedInstance().outputVolume);
    
    view.subviews[0].addSubview(upLabel);
    view.subviews[0].addSubview(downLabel);
  }

  private func upPressed() {
    upLabel.text = "Up";
  }
  
  private func upReleased() {
    upLabel.text = "";
  }
  
  private func downPressed() {
    downLabel.text = "Down";
  }
  
  private func downReleased() {
    downLabel.text = "";
  }

  private func volumeIncreased() {
    switch volumeState {
      case .None:
        volumeState = .Up;
        upPressed();
      case .Up:
        break;
      case .Down:
        volumeState = .Up;
        downReleased();
        upPressed();
    }
    workItem = DispatchWorkItem {
      self.volumeState = .None;
      self.upReleased();
      self.workItem = nil;
    };
  }
  
  private func volumeDecreased() {
    switch volumeState {
      case .None:
        volumeState = .Down;
        downPressed();
      case .Up:
        volumeState = .Down;
        upReleased();
        downPressed();
      case .Down:
        break;
    }
    workItem = DispatchWorkItem {
      self.volumeState = .None;
      self.downReleased();
      self.workItem = nil;
    };
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
      
    if newVolume > lastVolume || newVolume == 1.0 {
      volumeIncreased();
    } else if newVolume < lastVolume || newVolume == 0.0 {
      volumeDecreased();
    } else {
      return;
    }
    
    DispatchQueue.main.asyncAfter(deadline: .now() + ViewController.delay, execute: workItem!);
    lastVolume = newVolume;
  }
}
