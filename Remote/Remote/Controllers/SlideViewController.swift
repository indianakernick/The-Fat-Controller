//
//  SlideViewController.swift
//  Remote
//
//  Created by Indiana Kernick on 1/2/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import UIKit;

class SlideViewController: UIViewController, SocketManagerDelegate {
    private var socket = SocketManager();
    
    @IBOutlet weak var last: ButtonInput!;
    @IBOutlet weak var nextButton: ButtonInput!;
    @IBOutlet weak var previous: ButtonInput!;
    @IBOutlet weak var first: ButtonInput!;
    @IBOutlet weak var offlineCover: UIView!;
    
    private func setPressListener(for button: ButtonInput, with array: [UInt8]) {
        let data = Data(array);
        button.pressed = {
            self.socket.send(data);
        };
    }
    
    override func viewDidLoad() {
        super.viewDidLoad();
        
        socket.delegate = self;
        socket.connect();
        
        setPressListener(for: last, with: [CommandCode.keyClick.rawValue, Key.end.rawValue]);
        setPressListener(for: nextButton, with: [CommandCode.keyClick.rawValue, Key.rightArrow.rawValue]);
        setPressListener(for: previous, with: [CommandCode.keyClick.rawValue, Key.leftArrow.rawValue]);
        setPressListener(for: first, with: [CommandCode.keyClick.rawValue, Key.home.rawValue]);
    }
    
    func onlineStatusChanged(online: Bool) {
        if online {
            for view in view.subviews {
                view.isHidden = false;
            }
            UIView.animate(withDuration: 0.25, animations: {
                self.offlineCover.alpha = 0.0;
            }, completion: { finished in
                self.offlineCover.isHidden = true;
            });
        } else {
            offlineCover.isHidden = false;
            UIView.animate(withDuration: 0.25, animations: {
                self.offlineCover.alpha = 1.0;
            }, completion: { finished in
                for view in self.view.subviews {
                    view.isHidden = true;
                }
                self.offlineCover.isHidden = false;
            });
        }
    }
}
