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

fileprivate let mouseButtonRows = [
    (button: MouseButton.left, name: "Left"),
    (button: MouseButton.right, name: "Right"),
    (button: MouseButton.middle, name: "Middle"),
]

fileprivate let keyRows = [
    (key: Key.capsLock, name: "Caps Lock"),
    (key: Key.shift, name: "Shift"),
    (key: Key.control, name: "Control"),
    (key: Key.alt, name: "Alt"),
    (key: Key.meta, name: "Meta"),
    (key: Key.rightShift, name: "Right Shift"),
    (key: Key.rightControl, name: "Right Control"),
    (key: Key.rightAlt, name: "Right Alt"),
    (key: Key.rightMeta, name: "Right Meta"),
    (key: Key.fn, name: "Function"),
    
    (key: Key.return, name: "Return"),
    (key: Key.escape, name: "Escape"),
    (key: Key.delete, name: "Delete"),
    (key: Key.forwardDelete, name: "Forward Delete"),
    (key: Key.tab, name: "Tab"),
    (key: Key.space, name: "Space"),
    (key: Key.minus, name: "Minus"),
    (key: Key.equal, name: "Equal"),
    (key: Key.leftBracket, name: "Left Bracket"),
    (key: Key.rightBracket, name: "Right Bracket"),
    (key: Key.backslash, name: "Backslash"),
    (key: Key.semicolon, name: "Semicolon"),
    (key: Key.quote, name: "Quote"),
    (key: Key.grave, name: "Grave"),
    (key: Key.comma, name: "Comma"),
    (key: Key.period, name: "Period"),
    (key: Key.slash, name: "Slash"),
    
    (key: Key.upArrow, name: "Up Arrow"),
    (key: Key.rightArrow, name: "Right Arrow"),
    (key: Key.downArrow, name: "Down Arrow"),
    (key: Key.leftArrow, name: "Left Arrow"),
    (key: Key.pageUp, name: "Page Up"),
    (key: Key.pageDown, name: "Page Down"),
    (key: Key.home, name: "Home"),
    (key: Key.end, name: "End"),
    
    (key: Key.a, name: "A"),
    (key: Key.b, name: "B"),
    (key: Key.c, name: "C"),
    (key: Key.d, name: "D"),
    (key: Key.e, name: "E"),
    (key: Key.f, name: "F"),
    (key: Key.g, name: "G"),
    (key: Key.h, name: "H"),
    (key: Key.i, name: "I"),
    (key: Key.j, name: "J"),
    (key: Key.k, name: "K"),
    (key: Key.l, name: "L"),
    (key: Key.m, name: "M"),
    (key: Key.n, name: "N"),
    (key: Key.o, name: "O"),
    (key: Key.p, name: "P"),
    (key: Key.q, name: "Q"),
    (key: Key.r, name: "R"),
    (key: Key.s, name: "S"),
    (key: Key.t, name: "T"),
    (key: Key.u, name: "U"),
    (key: Key.v, name: "V"),
    (key: Key.w, name: "W"),
    (key: Key.x, name: "X"),
    (key: Key.y, name: "Y"),
    (key: Key.z, name: "Z"),
    
    (key: Key.n0, name: "0"),
    (key: Key.n1, name: "1"),
    (key: Key.n2, name: "2"),
    (key: Key.n3, name: "3"),
    (key: Key.n4, name: "4"),
    (key: Key.n5, name: "5"),
    (key: Key.n6, name: "6"),
    (key: Key.n7, name: "7"),
    (key: Key.n8, name: "8"),
    (key: Key.n9, name: "9"),
    
    (key: Key.keypad0, name: "Keypad 0"),
    (key: Key.keypad1, name: "Keypad 1"),
    (key: Key.keypad2, name: "Keypad 2"),
    (key: Key.keypad3, name: "Keypad 3"),
    (key: Key.keypad4, name: "Keypad 4"),
    (key: Key.keypad5, name: "Keypad 5"),
    (key: Key.keypad6, name: "Keypad 6"),
    (key: Key.keypad7, name: "Keypad 7"),
    (key: Key.keypad8, name: "Keypad 8"),
    (key: Key.keypad9, name: "Keypad 9"),
    
    (key: Key.keypadClear, name: "Keypad Clear"),
    (key: Key.keypadEquals, name: "Keypad Equals"),
    (key: Key.keypadDivide, name: "Keypad Divide"),
    (key: Key.keypadMultiply, name: "Keypad Multiply"),
    (key: Key.keypadMinus, name: "Keypad Minus"),
    (key: Key.keypadPlus, name: "Keypad Plus"),
    (key: Key.keypadEnter, name: "Keypad Enter"),
    (key: Key.keypadDecimal, name: "Keypad Decimal"),
    
    (key: Key.f1, name: "F1"),
    (key: Key.f2, name: "F2"),
    (key: Key.f3, name: "F3"),
    (key: Key.f4, name: "F4"),
    (key: Key.f5, name: "F5"),
    (key: Key.f6, name: "F6"),
    (key: Key.f7, name: "F7"),
    (key: Key.f8, name: "F8"),
    (key: Key.f9, name: "F9"),
    (key: Key.f10, name: "F10"),
    (key: Key.f11, name: "F11"),
    (key: Key.f12, name: "F12"),
    
    (key: Key.fastForward, name: "Fast-forward"),
    (key: Key.rewind, name: "Rewind"),
    (key: Key.playPause, name: "Play/pause"),
    (key: Key.volumeUp, name: "Volume Up"),
    (key: Key.volumeDown, name: "Volume Down"),
    (key: Key.mute, name: "Mute"),
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
                return mouseButtonRows.count
            } else {
                return keyRows.count
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
                return makeWhiteString(string: mouseButtonRows[row].name)
            } else {
                return makeWhiteString(string: keyRows[row].name)
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
        display: "Mouse down, Left",
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
    @IBOutlet weak var appendDown: ButtonInput!
    @IBOutlet weak var appendUp: ButtonInput!
    
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
            argumentName = mouseButtonRows[column1].name
            argumentByte = mouseButtonRows[column1].button.rawValue
        } else {
            argumentName = keyRows[column1].name
            argumentByte = keyRows[column1].key.rawValue
        }
        
        let display = commandName + ", " + argumentName
        var data = [commandByte, argumentByte]
        if commandByte != CommandCode.keyClick.rawValue && commandByte != CommandCode.keyUp.rawValue {
            data.append(0);
        }
        let listDelegate = tableView.dataSource as! CommandListDelegate
        listDelegate.rows.append(CommandRow(display: display, data: data))
        let count = listDelegate.rows.count
        tableView.insertRows(at: [IndexPath(row: count - 1, section: 0)], with: .fade)
    }
}
