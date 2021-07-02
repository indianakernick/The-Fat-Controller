//
//  SettingsVC.swift
//  Remote
//
//  Created by Indiana Kernick on 4/2/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import UIKit
import CryptoKit

class SettingsVC: UITableViewController, UITextFieldDelegate, NavigationChild, ScannerDelegate {
    private var socket: SocketManager!
    private var online = false
    
    // --- Interface Builder --- //
    
    @IBOutlet weak var hostNameField: UITextField!
    @IBOutlet weak var statusCell: UITableViewCell!
    @IBOutlet var statusIndicator: UIActivityIndicatorView!
    @IBOutlet weak var statusLabel: UILabel!
    @IBOutlet var lowLatencySwitch: UISwitch!
    @IBOutlet var secureSwitch: UISwitch!
    
    @IBAction func lowLatencyToggled() {
        Storage.setLowLatencyMode(lowLatencySwitch.isOn)
        socket.setLowLatencyMode(enabled: lowLatencySwitch.isOn)
    }
    
    @IBAction func secureToggled() {
        if secureSwitch.isOn {
            socket.setSecureMode(enabled: true)
            performSegue(withIdentifier: "Scan", sender: self)
        } else {
            socket.setSecureMode(enabled: false)
        }
    }
    
    // --- UIViewController --- //
    
    override func viewDidLoad() {
        super.viewDidLoad()
        hostNameField.delegate = self
        onlineStatusChanged(online: online)
    }
    
    override func viewWillAppear(_ animated: Bool) {
        super.viewWillAppear(animated)
        onlineStatusChanged(online: online)
        hostNameField.text = Storage.getHostName()
        lowLatencySwitch.isOn = Storage.getLowLatencyMode()
    }
    
    override var supportedInterfaceOrientations: UIInterfaceOrientationMask {
        return .portrait
    }
    
    override func prepare(for segue: UIStoryboardSegue, sender: Any?) {
        super.prepare(for: segue, sender: sender)
        
        if let dest = segue.destination as? ScannerVC {
            dest.delegate = self
        }
    }
    
    // --- UITextFieldDelegate --- //
    
    func textFieldShouldReturn(_ textField: UITextField) -> Bool {
        if let text = hostNameField.text {
            Storage.setHostName(text)
            socket.connectTo(host: text)
        }
        view.endEditing(true)
        return true
    }

    // --- NavigationChild --- //
    
    func onlineStatusChanged(online: Bool) {
        self.online = online
        if statusCell == nil {
            return
        }
        if online {
            statusLabel.text = "Connected"
            statusIndicator.stopAnimating()
            statusCell.accessoryView = nil
        } else {
            statusLabel.text = ""
            statusIndicator.startAnimating()
            statusCell.accessoryView = statusIndicator
        }
    }
    
    func onlineStatusInitial(online: Bool) {
        self.online = online
    }
    
    func setSocket(_ socket: SocketManager) {
        self.socket = socket
    }
    
    // --- ScannerDelegate --- //
    
    func scanDidSucceed(key: SymmetricKey) {
        socket.setSecureKey(key: key)
    }
    
    func scanDidFail() {
        secureSwitch.setOn(false, animated: true)
        socket.setSecureMode(enabled: false) // necessary?
        print("fail")
        // alert box?
    }
    
    func scanWasCancelled() {
        secureSwitch.setOn(false, animated: true)
        socket.setSecureMode(enabled: false) // necessary?
    }
}
