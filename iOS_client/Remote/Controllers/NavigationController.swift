//
//  NavigationController.swift
//  Remote
//
//  Created by Indiana Kernick on 3/2/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import UIKit

protocol NavigationChild: SocketManagerDelegate {
    func socketStatusInitial(_ status: SocketStatus)
    func setSocket(_ socket: SocketManager)
}

class NavigationController: UINavigationController, SocketManagerDelegate {
    private var socket = SocketManager()
    private var navChildren = [NavigationChild]()
    
    // --- NavigationController --- //
    
    func addNavChild(_ vc: UIViewController) {
        if let child = vc as? NavigationChild {
            if !navChildren.contains(where: { navChild in navChild === child }) {
                navChildren.append(child)
                child.setSocket(socket)
                child.socketStatusChanged(socket.getStatus())
            }
        }
    }
    
    // --- UIViewController --- //
    
    override func viewDidLoad() {
        super.viewDidLoad()
        socket.delegate = self
        socket.setLowLatencyMode(enabled: Storage.getLowLatencyMode())
        socket.setSecureMode(enabled: Storage.getSecureMode())
        socket.connectTo(host: Storage.getHostName())
        (children[0] as! ControllersVC).setNav(self)
    }
    
    override var supportedInterfaceOrientations: UIInterfaceOrientationMask {
        topViewController?.supportedInterfaceOrientations ?? .allButUpsideDown
    }
    
    override func viewWillTransition(to size: CGSize, with coordinator: UIViewControllerTransitionCoordinator) {
        super.viewWillTransition(to: size, with: coordinator);
        if size.width < size.height {
            setNavigationBarHidden(false, animated: true);
        } else {
            setNavigationBarHidden(true, animated: true);
        }
    }
    
    // --- SocketManagerDelegate --- //
    
    func socketStatusChanged(_ status: SocketStatus) {
        for child in navChildren {
            child.socketStatusChanged(status)
        }
    }
}
