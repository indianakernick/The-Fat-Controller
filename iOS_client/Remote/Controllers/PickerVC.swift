//
//  CommandVC.swift
//  Remote
//
//  Created by Indiana Kernick on 25/6/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import UIKit

protocol PickerDelegate: AnyObject {
    func didUpdate(value: UInt8)
}

class PickerVC: UIViewController, UIPickerViewDataSource, UIPickerViewDelegate {
    @IBOutlet weak var picker: UIPickerView!
    
    private var value: UInt8! = nil
    private var cases: [String]! = nil
    private weak var delegate: PickerDelegate? = nil
    
    // --- PickerVC --- //
    
    func setValue<E: Enum>(_ value: E) {
        self.value = value.rawValue
        if cases == nil {
            cases = []
        }
        cases.removeAll()
        cases.reserveCapacity(E.allCases.count)
        for e in E.allCases {
            cases.append(e.description)
        }
    }
    
    func setDelegate(_ delegate: PickerDelegate) {
        self.delegate = delegate
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
            delegate?.didUpdate(value: value)
        }
    }
    
    override func viewWillAppear(_ animated: Bool) {
        super.viewWillAppear(animated)
        picker.selectRow(Int(value), inComponent: 0, animated: false)
    }
    
    // --- UIPickerViewDataSource --- //
    
    func numberOfComponents(in pickerView: UIPickerView) -> Int {
        1
    }
    
    func pickerView(_ pickerView: UIPickerView, numberOfRowsInComponent component: Int) -> Int {
        cases.count
    }
    
    // --- UIPickerViewDelegate --- //
    
    func pickerView(_ pickerView: UIPickerView, titleForRow row: Int, forComponent component: Int) -> String? {
        cases[row]
    }
    
    func pickerView(_ pickerView: UIPickerView, didSelectRow row: Int, inComponent component: Int) {
        value = UInt8(row)
    }
}
