//
//  TabBarController.swift
//  Remote
//
//  Created by Indiana Kernick on 3/2/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import UIKit

protocol TakeSocket {
    func takeSocket(_ socket: SocketManager)
}

class TabBarController: UITabBarController, UITabBarControllerDelegate, SocketManagerDelegate {
    private var socket = SocketManager()
    private var previouslySelected: UIViewController?
    
    override func viewDidLoad() {
        super.viewDidLoad()
        delegate = self
        for controller in viewControllers! {
            (controller as? TakeSocket)?.takeSocket(socket)
        }
        socket.delegate = self
        socket.connectTo(host: UserDefaults.standard.string(forKey: StorageKeys.hostName) ?? "")
    }
    
    // The currently selected index should probably be stored in NSUserActivity
    // but getting that set up seems like a bit of a pain when all I want to do
    // is store an integer.
    
    override func viewWillAppear(_ animated: Bool) {
        selectedIndex = UserDefaults.standard.integer(forKey: StorageKeys.selectedTabIndex)
    }
    
    override func viewWillDisappear(_ animated: Bool) {
        UserDefaults.standard.set(selectedIndex, forKey: StorageKeys.selectedTabIndex)
    }
    
    override func viewWillTransition(to: CGSize, with: UIViewControllerTransitionCoordinator) {
        if to.width > to.height {
            tabBar.isHidden = true
        } else {
            tabBar.isHidden = false
        }
    }
    
    func onlineStatusChanged(online: Bool) {
        for controller in viewControllers! {
            (controller as? SocketManagerDelegate)?.onlineStatusChanged(online: online)
        }
    }
    
    func tabBarController(_ tabBarController: UITabBarController, didSelect viewController: UIViewController) {
        if viewController == viewControllers![3] && viewController == previouslySelected {
            viewController.performSegue(withIdentifier: "tapConfig", sender: viewController)
        }
        previouslySelected = viewController
    }
    
    // Makes it possible to have more than 5 tabs.
    // https://stackoverflow.com/a/40147148/4093378
    // It's a little bit cramped with 6 tabs so if we ever need 7, we should do
    // the right thing and show the "More" button.
    override var traitCollection: UITraitCollection {
        let realTraits = super.traitCollection
        let lieTrait = UITraitCollection.init(horizontalSizeClass: .regular)
        return UITraitCollection(traitsFrom: [realTraits, lieTrait])
    }
}
