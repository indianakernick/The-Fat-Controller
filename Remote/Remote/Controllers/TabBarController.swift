//
//  TabBarController.swift
//  Remote
//
//  Created by Indiana Kernick on 3/2/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import UIKit;

class TabBarController: UITabBarController {
    override func viewWillTransition(to: CGSize, with: UIViewControllerTransitionCoordinator) {
        if to.width > to.height {
            tabBar.isHidden = true;
        } else {
            tabBar.isHidden = false;
        }
    }
}
