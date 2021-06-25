//
//  CommandVC.swift
//  Remote
//
//  Created by Indiana Kernick on 25/6/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import UIKit

class CommandVC: UIViewController, UIPickerViewDataSource, UIPickerViewDelegate {
    @IBOutlet weak var picker: UIPickerView!
    
    private var commandCode: CommandCode! = nil
    
    func setCommandCode(_ code: CommandCode) {
        commandCode = code
    }
    
    // --- UIViewController --- //
    
    override func viewDidLoad() {
        super.viewDidLoad()
        picker.dataSource = self
        picker.delegate = self
    }
    
    override func viewWillDisappear(_ animated: Bool) {
        super.viewWillDisappear(animated)
        if isMovingFromParent {
            // Having to pass things back and forth like this is really awful.
            // Vue makes things like this so much simpler.
            let last = (parent as! NavigationController).viewControllers.last
            if let dest = last as? EditCommandVC {
                let row = picker.selectedRow(inComponent: 0)
                dest.setCommandCode(CommandCode.init(rawValue: UInt8(row))!)
            }
        }
    }
    
    override func viewWillAppear(_ animated: Bool) {
        super.viewWillAppear(animated)
        picker.selectRow(Int(commandCode.rawValue), inComponent: 0, animated: false)
    }
    
    // --- UIPickerViewDataSource --- //
    
    func numberOfComponents(in pickerView: UIPickerView) -> Int {
        1
    }
    
    func pickerView(_ pickerView: UIPickerView, numberOfRowsInComponent component: Int) -> Int {
        CommandCode.allCases.count
    }
    
    // --- UIPickerViewDelegate --- //
    
    func pickerView(_ pickerView: UIPickerView, titleForRow row: Int, forComponent component: Int) -> String? {
        CommandCode.allCases[row].description
    }
}
