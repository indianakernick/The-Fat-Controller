//
//  TabBarController.swift
//  Remote
//
//  Created by Indiana Kernick on 3/2/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import UIKit;

protocol TakeSocket {
    func takeSocket(_ socket: SocketManager);
}

class TabBarController: UITabBarController, UITabBarControllerDelegate, SocketManagerDelegate {
    private var socket = SocketManager();
    private var previouslySelected: UIViewController?;
    
    override func viewDidLoad() {
        super.viewDidLoad();
        delegate = self;
        for controller in viewControllers! {
            (controller as? TakeSocket)?.takeSocket(socket);
        }
        socket.delegate = self;
        socket.connectTo(host: UserDefaults.standard.string(forKey: StorageKeys.hostName) ?? "");
    }
    
    override func viewWillTransition(to: CGSize, with: UIViewControllerTransitionCoordinator) {
        if to.width > to.height {
            tabBar.isHidden = true;
        } else {
            tabBar.isHidden = false;
        }
    }
    
    func onlineStatusChanged(online: Bool) {
        for controller in viewControllers! {
            (controller as? SocketManagerDelegate)?.onlineStatusChanged(online: online);
        }
    }
    
    func tabBarController(_ tabBarController: UITabBarController, didSelect viewController: UIViewController) {
        if viewController == viewControllers![3] && viewController == previouslySelected {
            viewController.performSegue(withIdentifier: "tapConfig", sender: viewController);
        }
        previouslySelected = viewController;
    }
}
