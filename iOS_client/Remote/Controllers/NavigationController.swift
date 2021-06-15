//
//  NavigationController.swift
//  Remote
//
//  Created by Indiana Kernick on 3/2/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import UIKit

protocol TakeSocket {
    func takeSocket(_ socket: SocketManager)
}

class NavigationController: UINavigationController, SocketManagerDelegate {
    private var socket = SocketManager()
    private var previouslySelected: UIViewController?
    
    override func viewDidLoad() {
        super.viewDidLoad()
        //delegate = self
        for controller in viewControllers {
            (controller as? TakeSocket)?.takeSocket(socket)
        }
        socket.delegate = self
        socket.connectTo(host: UserDefaults.standard.string(forKey: StorageKeys.hostName) ?? "")
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
    
    func onlineStatusChanged(online: Bool) {
        for controller in viewControllers {
            (controller as? SocketManagerDelegate)?.onlineStatusChanged(online: online)
        }
    }
}
