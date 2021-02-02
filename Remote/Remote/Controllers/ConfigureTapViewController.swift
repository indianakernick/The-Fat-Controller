//
//  ConfigureTapViewController.swift
//  Remote
//
//  Created by Indiana Kernick on 2/2/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import UIKit;

fileprivate let commandNames = [
    "Mouse down",
    "Mouse up",
    "Mouse click",
    "Key down",
    "Key up",
    "Key click"
];

fileprivate let mouseButtonNames = [
    "Left",
    "Right",
    "Middle"
];

fileprivate let keyNames = [
    "A",
    "B",
    "C",
    "D",
    "E",
    "F"
];

class ConfigureTapViewController: UIViewController, UIPickerViewDataSource, UIPickerViewDelegate {
    @IBOutlet weak var downCommands: UITableView!;
    @IBOutlet weak var upCommands: UITableView!;
    @IBOutlet weak var commandPicker: UIPickerView!;
    
    private var mouseCommand = true;

    override func viewDidLoad() {
        super.viewDidLoad();
        
        commandPicker.dataSource = self;
        commandPicker.delegate = self;
        commandPicker.reloadAllComponents()
    }
    
    func numberOfComponents(in pickerView: UIPickerView) -> Int {
        return 2;
    }
    
    func pickerView(_ pickerView: UIPickerView, numberOfRowsInComponent component: Int) -> Int {
        if component == 0 {
            return commandNames.count;
        } else if component == 1 {
            if mouseCommand {
                return mouseButtonNames.count;
            } else {
                return keyNames.count;
            }
        } else {
            return 0;
        }
    }
    
    func pickerView(_ pickerView: UIPickerView, titleForRow row: Int, forComponent component: Int) -> String? {
        if component == 0 {
            return commandNames[row];
        } else if component == 1 {
            if mouseCommand {
                return mouseButtonNames[row];
            } else {
                return keyNames[row];
            }
        } else {
            return nil;
        }
    }
    
    func pickerView(_ pickerView: UIPickerView, didSelectRow row: Int, inComponent component: Int) {
        if component == 0 {
            if row < 3 && !mouseCommand {
                mouseCommand = true;
                commandPicker.reloadComponent(1);
                commandPicker.selectRow(0, inComponent: 1, animated: false);
            } else if row >= 3 && mouseCommand {
                mouseCommand = false;
                commandPicker.reloadComponent(1);
                commandPicker.selectRow(0, inComponent: 1, animated: false);
            }
        }
    }
}
