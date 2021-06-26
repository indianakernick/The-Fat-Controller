//
//  EditCommandVC.swift
//  Remote
//
//  Created by Indiana Kernick on 25/6/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import UIKit

fileprivate enum ParameterType {
    case uint, int, key, mouseButton, char, string
}

fileprivate struct Parameter {
    let name: String?
    let type: ParameterType
}

// ASCII probably doesn't make much sense
// Should just use the Unicode commands
// That's going to be a real hassle

fileprivate let parameters: [[Parameter]] = [
    [Parameter(name: "DELAY (MILLISECONDS)", type: .uint)],
    [Parameter(name: nil, type: .key)],
    [Parameter(name: nil, type: .key)],
    [Parameter(name: nil, type: .key)],
    [Parameter(name: "X (PIXELS)", type: .int), Parameter(name: "Y (PIXELS)", type: .int)],
    [Parameter(name: "X (PIXELS)", type: .int), Parameter(name: "Y (PIXELS)", type: .int)],
    [Parameter(name: "X (PIXELS)", type: .int), Parameter(name: "Y (PIXELS)", type: .int)],
    [Parameter(name: nil, type: .mouseButton)],
    [Parameter(name: nil, type: .mouseButton)],
    [Parameter(name: nil, type: .mouseButton)],
    [Parameter(name: "CHARACTER", type: .char)],
    [Parameter(name: "CHARACTER", type: .char)],
    [Parameter(name: "CHARACTER", type: .char)],
    [Parameter(name: "STRING", type: .string)],
    [Parameter(name: "CHARACTER", type: .char)],
    [Parameter(name: "CHARACTER", type: .char)],
    [Parameter(name: "CHARACTER", type: .char)],
    [Parameter(name: "STRING", type: .string)]
]

class EditCommandVC: UITableViewController, PickerDelegate {
    private var commandCode = CommandCode.mouseClick
    private var key = Key.space
    private var mouseButton = MouseButton.left
    
    func setCommandCode(_ code: CommandCode) {
        commandCode = code
    }
    
    private func getParameter(index: Int) -> Parameter {
        parameters[Int(commandCode.rawValue)][index]
    }
    
    // --- UITableViewController --- //
    
    override func prepare(for segue: UIStoryboardSegue, sender: Any?) {
        super.prepare(for: segue, sender: sender)
        if let dest = segue.destination as? PickerVC {
            dest.setDelegate(self)
            if let cell = sender as? UITableViewCell {
                if cell.reuseIdentifier == "Command" {
                    dest.initialize(value: commandCode, id: 0, name: "Command")
                } else if cell.reuseIdentifier == "Key" {
                    dest.initialize(value: key, id: 1, name: "Key")
                } else if cell.reuseIdentifier == "MouseButton" {
                    dest.initialize(value: mouseButton, id: 2, name: "Mouse Button")
                }
            }
        }
    }
    
    override func viewWillAppear(_ animated: Bool) {
        super.viewWillAppear(animated)
        tableView.reloadData()
    }
    
    override func numberOfSections(in: UITableView) -> Int {
        if commandCode.rawValue < parameters.count {
            return 1 + parameters[Int(commandCode.rawValue)].count
        } else {
            return 1
        }
    }
    
    override func tableView(_ tableView: UITableView, numberOfRowsInSection section: Int) -> Int {
        1
    }
    
    override func tableView(_ tableView: UITableView, cellForRowAt indexPath: IndexPath) -> UITableViewCell {
        if indexPath.section == 0 {
            let cell = tableView.dequeueReusableCell(withIdentifier: "Command", for: indexPath)
            cell.detailTextLabel!.text = commandCode.description
            return cell
        } else {
            switch getParameter(index: indexPath.section - 1).type {
            case .int:
                fallthrough
            case .uint:
                let cell = tableView.dequeueReusableCell(withIdentifier: "TextInput", for: indexPath) as! TextInputCell
                cell.textInput.setIndent(tableView.separatorInset.left)
                // might do something really sneaky and use the decimal point as a +/- button!
                cell.textInput.keyboardType = .decimalPad
                return cell
            case .key:
                let cell = tableView.dequeueReusableCell(withIdentifier: "Key", for: indexPath)
                cell.detailTextLabel!.text = key.description
                return cell
            case .mouseButton:
                let cell = tableView.dequeueReusableCell(withIdentifier: "MouseButton", for: indexPath)
                cell.detailTextLabel!.text = mouseButton.description
                return cell
            case .char:
                fallthrough
            case .string:
                let cell = tableView.dequeueReusableCell(withIdentifier: "TextInput", for: indexPath) as! TextInputCell
                cell.textInput.setIndent(tableView.separatorInset.left)
                cell.textInput.keyboardType = .default
                return cell
            }
        }
    }
    
    override func tableView(_ tableView: UITableView, titleForHeaderInSection section: Int) -> String? {
        section > 0 ? getParameter(index: section - 1).name : nil
    }
    
    // --- PickerDelegate --- //
    
    func didUpdate(value: UInt8, id: Int) {
        if id == 0 {
            commandCode = CommandCode(rawValue: value)!
        } else if id == 1 {
            key = Key(rawValue: value)!
        } else if id == 2 {
            mouseButton = MouseButton(rawValue: value)!
        }
    }
}
