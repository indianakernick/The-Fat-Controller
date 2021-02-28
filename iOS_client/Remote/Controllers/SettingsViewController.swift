//
//  SettingsViewController.swift
//  Remote
//
//  Created by Indiana Kernick on 4/2/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import UIKit

class SettingsViewController: UIViewController, UITextFieldDelegate, SocketManagerDelegate, TakeSocket {
    static let transitionDuration = 0.25
    
    @IBOutlet weak var hostAddressField: UITextField!
    @IBOutlet weak var hostPortField: UITextField!
    @IBOutlet weak var statusLabel: UILabel!
    
    private var socket: SocketManager!
    private var online = false
    private var port: UInt16 = 0
    
    func takeSocket(_ socket: SocketManager) {
        self.socket = socket
    }
    
    override func viewDidLoad() {
        super.viewDidLoad()
        
        statusLabel.layer.masksToBounds = true
        statusLabel.layer.cornerRadius = 8
        
        initField(hostAddressField)
        initField(hostPortField)
        
        let rect = CGRect(x: 0, y: 0, width: 8, height: hostAddressField.frame.size.height)
        
        let leftView = UIView(frame: rect)
        leftView.backgroundColor = hostAddressField.backgroundColor
        hostAddressField.leftView = leftView
        hostAddressField.leftViewMode = .always
        
        let rightView = UIView(frame: rect)
        rightView.backgroundColor = hostPortField.backgroundColor
        hostPortField.rightView = rightView
        hostPortField.rightViewMode = .always
        
        view.addGestureRecognizer(UITapGestureRecognizer(
            target: self, action: #selector(self.hideKeyboard)
        ))
    }
    
    override func viewWillAppear(_ animated: Bool) {
        onlineStatusChanged(online: online)
        hostAddressField.text = UserDefaults.standard.string(forKey: StorageKeys.hostAddress)
        let port = UInt16(UserDefaults.standard.integer(forKey: StorageKeys.hostPort))
        self.port = port == 0 ? StorageDefaults.hostPort : port
        hostPortField.text = "\(self.port)"
    }
    
    func textFieldShouldReturn(_ textField: UITextField) -> Bool {
        if textField == hostAddressField, let text = hostAddressField.text {
            UserDefaults.standard.set(text, forKey: StorageKeys.hostAddress)
            socket.connectTo(host: text, port: port)
        }
        view.endEditing(true)
        return true
    }
    
    func textFieldDidEndEditing(_ textField: UITextField) {
        if textField == hostPortField, let text = hostPortField.text {
            if let int = UInt16(text), int != 0 {
                port = int
                UserDefaults.standard.set(port, forKey: StorageKeys.hostPort)
                socket.connectTo(host: hostAddressField.text ?? "", port: port)
            } else {
                hostPortField.text = "\(port)"
            }
        }
    }
    
    func onlineStatusChanged(online: Bool) {
        self.online = online
        if statusLabel == nil {
            return
        }
        if online {
            statusLabel.text = "Connected"
            UIView.animate(withDuration: SettingsViewController.transitionDuration, animations: {
                self.statusLabel.layer.backgroundColor = Colors.green
            })
        } else {
            statusLabel.text = "Disconnected"
            UIView.animate(withDuration: SettingsViewController.transitionDuration, animations: {
                self.statusLabel.layer.backgroundColor = Colors.red
            })
        }
    }
    
    private func initField(_ field: UITextField) {
        field.delegate = self
        field.layer.masksToBounds = true
        field.layer.cornerRadius = 8
        field.overrideUserInterfaceStyle = .dark
    }
    
    @objc private func hideKeyboard() {
        hostAddressField.resignFirstResponder()
        hostPortField.resignFirstResponder()
    }
}
