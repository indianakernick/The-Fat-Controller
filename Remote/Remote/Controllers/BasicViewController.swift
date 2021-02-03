//
//  BasicViewController.swift
//  Remote
//
//  Created by Indiana Kernick on 1/2/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import UIKit;

class BasicViewController: UIViewController, SocketManagerDelegate {
    private var socket: SocketManager!;
    
    @IBOutlet weak var offlineCover: UIView!;
    
    func setSocket(_ sock: SocketManager) {
        socket = sock;
    }
    
    func send(_ data: Data) {
        socket.send(data);
    }
    
    func makeListener(with array: [UInt8]) -> () -> () {
        let data = Data(array);
        return { [weak self] in
            self!.socket.send(data);
        };
    }
    
    func setPressListener(for button: ButtonInput, with array: [UInt8]) {
        button.pressed = makeListener(with: array);
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
