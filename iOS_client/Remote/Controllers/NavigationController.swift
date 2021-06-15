//
//  NavigationController.swift
//  Remote
//
//  Created by Indiana Kernick on 3/2/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import UIKit

protocol NavigationChild: SocketManagerDelegate {
    func onlineStatusInitial(online: Bool)
    func takeSocket(_ socket: SocketManager)
}

class NavigationController: UINavigationController, SocketManagerDelegate {
    private var socket = SocketManager()
    private var navChildren = [NavigationChild]()
    
    override func viewDidLoad() {
        super.viewDidLoad()
        socket.delegate = self
        socket.connectTo(host: UserDefaults.standard.string(forKey: StorageKeys.hostName) ?? "")
        (children[0] as! ControllersViewController).setNav(self)
    }
    
    // Not sure if this is necessary or how it would be done with a navigation
    // controller.
    
    // The currently selected index should probably be stored in NSUserActivity
    // but getting that set up seems like a bit of a pain when all I want to do
    // is store an integer.
    
    /*
     override func viewWillAppear(_ animated: Bool) {
        selectedIndex = UserDefaults.standard.integer(forKey: StorageKeys.selectedTabIndex)
    }
    
    override func viewWillDisappear(_ animated: Bool) {
        UserDefaults.standard.set(selectedIndex, forKey: StorageKeys.selectedTabIndex)
    }
    */
    
    func addNavChild(_ vc: UIViewController) {
        if let child = vc as? NavigationChild {
            if !navChildren.contains(where: { navChild in navChild === child }) {
                navChildren.append(child)
                child.takeSocket(socket)
                child.onlineStatusInitial(online: socket.getOnlineStatus())
            }
        }
    }
    
    func onlineStatusChanged(online: Bool) {
        for child in navChildren {
            child.onlineStatusChanged(online: online)
        }
    }
}
