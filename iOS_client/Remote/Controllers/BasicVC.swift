//
//  BasicVC.swift
//  Remote
//
//  Created by Indiana Kernick on 1/2/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import UIKit

class BasicVC: UIViewController, NavigationChild {
    private var socket: SocketManager!
    private var online = false
    
    // --- Interface Builder --- //
    
    @IBOutlet weak var offlineCover: UIView!
    
    // --- BasicVC --- //
    
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
    
    // --- UIViewController --- //
    
    override func viewWillAppear(_ animated: Bool) {
        super.viewWillAppear(animated)
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
    
    // --- NavigationChild --- //
    
    func socketStatusChanged(_ status: SocketStatus) {
        online = status == .connected
        
        if self.offlineCover == nil {
            return
        }
        
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
    
    func socketStatusInitial(_ status: SocketStatus) {
        online = status == .connected
    }
    
    func setSocket(_ socket: SocketManager) {
        self.socket = socket
    }
}
