//
//  TapVC.swift
//  Remote
//
//  Created by Indiana Kernick on 2/2/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import Foundation

fileprivate func dataFromPlist(_ plist: [Any]) -> Data? {
    var buf = Data()
    for item in plist {
        guard let dict = item as? [String: Any] else { return nil }
        guard let data = dict["data"] as? Data else { return nil }
        buf.append(data)
    }
    return buf
}

class TapVC: BasicVC {
    @IBOutlet weak var tap: TapInput!
    
    private var downData = CommandData.mouseDown(MouseButton.left)
    private var upData = CommandData.mouseUp(MouseButton.left)
    
    static weak var instance: TapVC?
    
    override func viewDidLoad() {
        super.viewDidLoad()
        TapVC.instance = self
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
        if
            let downPlist = Storage.getTapDownCommandList(),
            let upPlist = Storage.getTapUpCommandList(),
            let downData = dataFromPlist(downPlist),
            let upData = dataFromPlist(upPlist)
        {
            self.downData = downData
            self.upData = upData
        }
    }
}
