//
//  BasicViewController.swift
//  Remote
//
//  Created by Indiana Kernick on 1/2/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import UIKit

class BasicViewController: UIViewController, NavigationChild {
    private var socket: SocketManager!
    private var online = false
    
    @IBOutlet weak var offlineCover: UIView!
    
    override func viewDidLayoutSubviews() {
        if online {
            for view in view.subviews {
                view.isHidden = false
            }
            offlineCover.isHidden = true
            offlineCover.alpha = 0.0
        } else {
            for view in view.subviews {
                view.isHidden = true
            }
            offlineCover.isHidden = false
            offlineCover.alpha = 1.0
        }
    }
    
    func send(_ data: Data) {
        socket.send(data)
    }
    
    func makeListener(with data: Data) -> () -> () {
        return { [weak self] in
            self!.socket.send(data)
        }
    }
    
    func setPressListener(for button: LabelButtonInput, with data: Data) {
        button.pressed = makeListener(with: data)
    }
    
    func onlineStatusChanged(online: Bool) {
        if online {
            for view in view.subviews {
                view.isHidden = false
            }
            UIView.animate(withDuration: Config.fadeAnimationDuration, animations: {
                self.offlineCover.alpha = 0.0
            }, completion: { finished in
                self.offlineCover.isHidden = true
            })
        } else {
            offlineCover.isHidden = false
            UIView.animate(withDuration: Config.fadeAnimationDuration, animations: {
                if self.offlineCover != nil {
                    self.offlineCover.alpha = 1.0
                }
            }, completion: { finished in
                for view in self.view.subviews {
                    view.isHidden = true
                }
                self.offlineCover.isHidden = false
            })
        }
    }
    
    func onlineStatusInitial(online: Bool) {
        self.online = online
    }
    
    func takeSocket(_ socket: SocketManager) {
        self.socket = socket
    }
}
