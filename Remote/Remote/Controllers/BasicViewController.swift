//
//  BasicViewController.swift
//  Remote
//
//  Created by Indiana Kernick on 1/2/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import UIKit;

class BasicViewController: UIViewController, SocketManagerDelegate {
    private var socket = SocketManager();
    
    @IBOutlet weak var offlineCover: UIView!;
    
    override func viewDidLoad() {
        super.viewDidLoad();
        socket.delegate = self;
        socket.connect();
    }
    
    func setPressListener(for button: ButtonInput, with array: [UInt8]) {
        let data = Data(array);
        button.pressed = {
            self.socket.send(data);
        };
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
