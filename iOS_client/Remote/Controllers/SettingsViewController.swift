//
//  SettingsViewController.swift
//  Remote
//
//  Created by Indiana Kernick on 4/2/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import UIKit

class SettingsViewController: UIViewController, UITextFieldDelegate, NavigationChild {
    @IBOutlet weak var hostNameField: UITextField!
    @IBOutlet weak var statusLabel: UILabel!
    
    private var socket: SocketManager!
    private var online = false
    
    override func viewDidLoad() {
        super.viewDidLoad()
        
        statusLabel.layer.masksToBounds = true
        statusLabel.layer.cornerRadius = 8
        
        hostNameField.delegate = self
        hostNameField.layer.masksToBounds = true
        hostNameField.layer.cornerRadius = 8
        
        let leftView = UIView(frame: CGRect(x: 0, y: 0, width: 8, height: hostNameField.frame.size.height))
        leftView.backgroundColor = hostNameField.backgroundColor
        hostNameField.leftView = leftView
        hostNameField.leftViewMode = .always
        
        if online {
            statusLabel.text = "Connected"
            statusLabel.layer.backgroundColor = Colors.green
        } else {
            statusLabel.text = "Disconnected"
            statusLabel.layer.backgroundColor = Colors.red
        }
    }
    
    override func viewWillAppear(_ animated: Bool) {
        onlineStatusChanged(online: online)
        hostNameField.text = UserDefaults.standard.string(forKey: StorageKeys.hostName)
    }
    
    func textFieldShouldReturn(_ textField: UITextField) -> Bool {
        if let text = hostNameField.text {
            UserDefaults.standard.set(text, forKey: StorageKeys.hostName)
            socket.connectTo(host: text)
        }
        view.endEditing(true)
        return true
    }

    func onlineStatusChanged(online: Bool) {
        self.online = online
        if statusLabel == nil {
            return
        }
        if online {
            statusLabel.text = "Connected"
            UIView.animate(withDuration: Config.fadeAnimationDuration, animations: {
                self.statusLabel.layer.backgroundColor = Colors.green
            })
        } else {
            statusLabel.text = "Disconnected"
            UIView.animate(withDuration: Config.fadeAnimationDuration, animations: {
                self.statusLabel.layer.backgroundColor = Colors.red
            })
        }
    }
    
    func onlineStatusInitial(online: Bool) {
        self.online = online
    }
    
    func takeSocket(_ socket: SocketManager) {
        self.socket = socket
    }
}
