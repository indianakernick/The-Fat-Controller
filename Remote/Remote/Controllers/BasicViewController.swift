//
//  BasicViewController.swift
//  Remote
//
//  Created by Indiana Kernick on 1/2/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import UIKit

class BasicViewController: UIViewController, SocketManagerDelegate, TakeSocket {
    private var socket: SocketManager!
    
    @IBOutlet weak var offlineCover: UIView!
    
    func takeSocket(_ socket: SocketManager) {
        self.socket = socket
    }
    
    func send(_ data: Data) {
        socket.send(data)
    }
    
    func makeListener(with data: Data) -> () -> () {
        return { [weak self] in
            self!.socket.send(data)
        }
    }
    
    func setPressListener(for button: ButtonInput, with data: Data) {
        button.pressed = makeListener(with: data)
    }
    
    func onlineStatusChanged(online: Bool) {
        if online {
            for view in view.subviews {
                view.isHidden = false
            }
            UIView.animate(withDuration: 0.25, animations: {
                self.offlineCover.alpha = 0.0
            }, completion: { finished in
                self.offlineCover.isHidden = true
            })
        } else {
            offlineCover.isHidden = false
            UIView.animate(withDuration: 0.25, animations: {
                self.offlineCover.alpha = 1.0
            }, completion: { finished in
                for view in self.view.subviews {
                    view.isHidden = true
                }
                self.offlineCover.isHidden = false
            })
        }
    }
    
    override var traitCollection: UITraitCollection {
        let realTraits = super.traitCollection
        let lieTrait = UITraitCollection.init(horizontalSizeClass: .compact)
        return UITraitCollection(traitsFrom: [realTraits, lieTrait])
    }
}
