//
//  ControllersVC.swift
//  Remote
//
//  Created by Indiana Kernick on 15/6/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import UIKit

class ControllersVC: UITableViewController, NavigationChild {
    private var nav: NavigationController!
    private var socket: SocketManager!
    
    private func update(_ status: SocketStatus) {
        if settingsStatus != nil {
            settingsStatus.text = SettingsVC.statusText(status: status, host: socket.getHost())
        }
    }
    
    // --- Interface Builder --- //
    
    @IBOutlet weak var settingsStatus: UILabel!
    
    // --- ControllersVC --- //
    
    func setNav(_ nav: NavigationController) {
        self.nav = nav
    }
    
    // --- UIViewController --- //
    
    override func viewDidLoad() {
        super.viewDidLoad()
        nav.addNavChild(self)
        update(socket.getStatus())
    }
    
    override func prepare(for segue: UIStoryboardSegue, sender: Any?) {
        nav.addNavChild(segue.destination)
    }
    
    override var supportedInterfaceOrientations: UIInterfaceOrientationMask {
        return .portrait
    }
    
    // --- NavigationChild --- //
    
    func socketStatusChanged(_ status: SocketStatus) {
        update(status)
    }
    
    func socketStatusInitial(_ status: SocketStatus) {
        update(status)
    }
    
    func setSocket(_ socket: SocketManager) {
        self.socket = socket
    }
}
