//
//  ControllersVC.swift
//  Remote
//
//  Created by Indiana Kernick on 15/6/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import UIKit

class ControllersVC: UITableViewController, NavigationChild {
    @IBOutlet weak var settingsStatus: UILabel!
    
    private var nav: NavigationController!
    private var socket: SocketManager!
    
    override func viewDidLoad() {
        super.viewDidLoad()
        nav.addNavChild(self)
        update()
    }
    
    override func prepare(for segue: UIStoryboardSegue, sender: Any?) {
        nav.addNavChild(segue.destination)
    }
    
    override var supportedInterfaceOrientations: UIInterfaceOrientationMask {
        return .portrait
    }
    
    func setNav(_ nav: NavigationController) {
        self.nav = nav
    }
    
    private func update() {
        if settingsStatus != nil {
            settingsStatus.text = socket.getOnlineHost() ?? "Not Connected"
        }
    }
    
    func onlineStatusChanged(online: Bool) {
        update()
    }
    
    func onlineStatusInitial(online: Bool) {
        update()
    }
    
    func setSocket(_ socket: SocketManager) {
        self.socket = socket
    }
}
