//
//  ConfigureTapViewController.swift
//  Remote
//
//  Created by Indiana Kernick on 2/2/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import UIKit

fileprivate let commandCodeRows = [
    (code: CommandCode.mouseDown, name: "Mouse down"),
    (code: CommandCode.mouseUp, name: "Mouse up"),
    (code: CommandCode.mouseClick, name: "Mouse click"),
    (code: CommandCode.keyDown, name: "Key down"),
    (code: CommandCode.keyUp, name: "Key up"),
    (code: CommandCode.keyClick, name: "Key click"),
]

fileprivate struct CommandRow {
    var display: String
    var data: [UInt8]
}

class CommandPickerDelegate: NSObject, UIPickerViewDataSource, UIPickerViewDelegate {
    private var mouseCommand = true
    
    func numberOfComponents(in pickerView: UIPickerView) -> Int {
        return 2
    }
    
    func pickerView(_ pickerView: UIPickerView, numberOfRowsInComponent component: Int) -> Int {
        if component == 0 {
            return commandCodeRows.count
        } else if component == 1 {
            if mouseCommand {
                return MouseButton.allCases.count
            } else {
                return Key.allCases.count
            }
        } else {
            return 0
        }
    }
    
    private func makeWhiteString(string: String) -> NSAttributedString {
        return NSAttributedString(
            string: string,
            attributes: [NSAttributedString.Key.foregroundColor: UIColor(cgColor: Colors.gray200)]
        )
    }
    
    func pickerView(_ pickerView: UIPickerView, attributedTitleForRow row: Int, forComponent component: Int) -> NSAttributedString? {
        if component == 0 {
            return makeWhiteString(string: commandCodeRows[row].name)
        } else if component == 1 {
            if mouseCommand {
                return makeWhiteString(string: MouseButton(rawValue: UInt8(row))!.description)
            } else {
                return makeWhiteString(string: Key(rawValue: UInt8(row))!.description)
            }
        } else {
            return nil
        }
    }
    
    func pickerView(_ pickerView: UIPickerView, didSelectRow row: Int, inComponent component: Int) {
        if component == 0 {
            if (row < 3 && !mouseCommand) || (row >= 3 && mouseCommand) {
                mouseCommand = !mouseCommand
                pickerView.reloadComponent(1)
                pickerView.selectRow(0, inComponent: 1, animated: false)
            }
        }
    }
}

class CommandListDelegate: NSObject, UITableViewDataSource {
    fileprivate var rows: [CommandRow] = [CommandRow(
        display: "\(CommandCode.mouseDown), \(MouseButton.left)",
        data: [CommandCode.mouseDown.rawValue, MouseButton.left.rawValue]
    )]
    
    func tableView(_ tableView: UITableView, numberOfRowsInSection section: Int) -> Int {
        return rows.count
    }
    
    func tableView(_ tableView: UITableView, cellForRowAt indexPath: IndexPath) -> UITableViewCell {
        let cell = tableView.dequeueReusableCell(withIdentifier: "CommandCell", for: indexPath)
        cell.textLabel!.textColor = UIColor(cgColor: Colors.gray200)
        // Sets color of reorder control.
        // Might want to consider setting this for the whole app.
        cell.overrideUserInterfaceStyle = .dark
        cell.textLabel!.text = rows[indexPath.row].display
        return cell
    }
    
    func tableView(_ tableView: UITableView, commit editingStyle: UITableViewCell.EditingStyle, forRowAt indexPath: IndexPath) {
        if editingStyle == .delete {
            rows.remove(at: indexPath.row)
            tableView.deleteRows(at: [indexPath], with: .fade)
        }
    }
    
    func tableView(_ tableView: UITableView, moveRowAt sourceIndexPath: IndexPath, to destinationIndexPath: IndexPath) {
        let item = rows[sourceIndexPath.row]
        rows.remove(at: sourceIndexPath.row)
        rows.insert(item, at: destinationIndexPath.row)
    }
}

fileprivate func rowsFromPlist(plist: [Any]) -> [CommandRow] {
    var rows: [CommandRow] = []
    for element in plist {
        let dict = element as! [String : Any]
        let display = dict["display"] as! String
        let data = dict["data"] as! [UInt8]
        rows.append(CommandRow(display: display, data: data))
    }
    return rows
}

fileprivate func rowsToPlist(rows: [CommandRow]) -> [Any] {
    var plist: [[String : Any]] = []
    for row in rows {
        plist.append(["display": row.display, "data": row.data])
    }
    return plist
}

class ConfigureTapViewController: UIViewController {
    @IBOutlet weak var downCommands: UITableView!
    @IBOutlet weak var upCommands: UITableView!
    @IBOutlet weak var commandPicker: UIPickerView!
    @IBOutlet weak var appendDown: LabelButtonInput!
    @IBOutlet weak var appendUp: LabelButtonInput!
    
    private var commandPickerDelegate = CommandPickerDelegate()
    private var downCommandsDelegate = CommandListDelegate()
    private var upCommandsDelegate = CommandListDelegate()

    override func viewDidLoad() {
        super.viewDidLoad()
        
        commandPicker.dataSource = commandPickerDelegate
        commandPicker.delegate = commandPickerDelegate
        commandPicker.reloadAllComponents()

        downCommands.dataSource = downCommandsDelegate
        downCommands.isEditing = true
        
        upCommands.dataSource = upCommandsDelegate
        upCommands.isEditing = true
        
        appendDown.pressed = { [weak self] in
            self!.appendTo(tableView: self!.downCommands)
        }
        
        appendUp.pressed = { [weak self] in
            self!.appendTo(tableView: self!.upCommands)
        }
    }
    
    override func viewWillAppear(_ animated: Bool) {
        super.viewWillAppear(animated)
        let downRows = UserDefaults.standard.array(forKey: StorageKeys.tapDownCommandList)
        let upRows = UserDefaults.standard.array(forKey: StorageKeys.tapUpCommandList)
        if downRows != nil && upRows != nil {
            downCommandsDelegate.rows = rowsFromPlist(plist: downRows!)
            upCommandsDelegate.rows = rowsFromPlist(plist: upRows!)
        }
        downCommands.reloadData()
        upCommands.reloadData()
    }
    
    override func viewWillDisappear(_ animated: Bool) {
        super.viewWillDisappear(animated)
        let downRows = rowsToPlist(rows: downCommandsDelegate.rows)
        let upRows = rowsToPlist(rows: upCommandsDelegate.rows)
        UserDefaults.standard.set(downRows, forKey: StorageKeys.tapDownCommandList)
        UserDefaults.standard.set(upRows, forKey: StorageKeys.tapUpCommandList)
        TapViewController.instance?.updateData()
    }
    
    private func appendTo(tableView: UITableView) {
        let column0 = commandPicker.selectedRow(inComponent: 0)
        let column1 = commandPicker.selectedRow(inComponent: 1)
        
        let commandName = commandCodeRows[column0].name
        let commandByte = commandCodeRows[column0].code.rawValue
        let argumentName: String
        let argumentByte: UInt8
        if column0 < 3 {
            argumentName = MouseButton(rawValue: UInt8(column1))!.description
            argumentByte = UInt8(column1)
        } else {
            argumentName = Key(rawValue: UInt8(column1))!.description
            argumentByte = UInt8(column1)
        }
        
        let display = commandName + ", " + argumentName
        let data = [commandByte, argumentByte]
        let listDelegate = tableView.dataSource as! CommandListDelegate
        listDelegate.rows.append(CommandRow(display: display, data: data))
        let count = listDelegate.rows.count
        tableView.insertRows(at: [IndexPath(row: count - 1, section: 0)], with: .fade)
    }
}
