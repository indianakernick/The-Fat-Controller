//
//  ControllersVC.swift
//  Remote
//
//  Created by Indiana Kernick on 15/6/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import UIKit

class ControllersVC: UITableViewController {
    private var nav: NavigationController!
    
    func setNav(_ nav: NavigationController) {
        self.nav = nav
    }
    
    override func prepare(for segue: UIStoryboardSegue, sender: Any?) {
        nav.addNavChild(segue.destination)
    }
    
    override var supportedInterfaceOrientations: UIInterfaceOrientationMask {
        return .portrait
    }
}
