//
//  TapViewController.swift
//  Remote
//
//  Created by Indiana Kernick on 2/2/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import Foundation

fileprivate func dataFromPlist(plist: [Any]) -> Data {
    var bytes: [UInt8] = []
    for element in plist {
        let dict = element as! [String : Any]
        bytes += dict["data"] as! [UInt8]
    }
    return Data(bytes)
}

class TapViewController: BasicViewController {
    @IBOutlet weak var tap: TapInput!
    
    private var downData = Command.mouseDown(MouseButton.left)
    private var upData = Command.mouseUp(MouseButton.left)
    
    static weak var instance: TapViewController?
    
    override func viewDidLoad() {
        super.viewDidLoad()
        TapViewController.instance = self
        tap.pressed = { [weak self] in
            self!.send(self!.downData)
        }
        tap.released = { [weak self] in
            self!.send(self!.upData)
        }
    }
    
    override func viewWillAppear(_ animated: Bool) {
        super.viewWillAppear(animated)
        updateData()
    }
    
    func updateData() {
        let downRows = UserDefaults.standard.array(forKey: StorageKeys.tapDownCommandList)
        let upRows = UserDefaults.standard.array(forKey: StorageKeys.tapUpCommandList)
        if downRows != nil && upRows != nil {
            downData = dataFromPlist(plist: downRows!)
            upData = dataFromPlist(plist: upRows!)
        }
    }
}
