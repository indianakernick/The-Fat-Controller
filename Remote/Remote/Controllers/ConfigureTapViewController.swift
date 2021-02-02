//
//  ConfigureTapViewController.swift
//  Remote
//
//  Created by Indiana Kernick on 2/2/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import UIKit;

fileprivate let commandCodeRows = [
    (code: CommandCode.mouseDown, name: "Mouse down"),
    (code: CommandCode.mouseUp, name: "Mouse up"),
    (code: CommandCode.mouseClick, name: "Mouse click"),
    (code: CommandCode.keyDown, name: "Key down"),
    (code: CommandCode.keyUp, name: "Key up"),
    (code: CommandCode.keyClick, name: "Key click"),
];

fileprivate let mouseButtonRows = [
    (button: MouseButton.left, name: "Left"),
    (button: MouseButton.right, name: "Right"),
    (button: MouseButton.middle, name: "Middle"),
];

fileprivate let keyRows = [
    (key: Key.capsLock, name: "Caps Lock"),
    (key: Key.control, name: "Control"),
    (key: Key.shift, name: "Shift"),
    (key: Key.option, name: "Option"),
    (key: Key.command, name: "Command"),
    (key: Key.rightControl, name: "Right Control"),
    (key: Key.rightShift, name: "Right Shift"),
    (key: Key.rightOption, name: "Right Option"),
    (key: Key.rightCommand, name: "Right Command"),
    (key: Key.function, name: "Function"),
    
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
    (key: Key.grave, name: "Tilde"),
    (key: Key.comma, name: "Comma"),
    (key: Key.period, name: "Period"),
    (key: Key.slash, name: "Slash"),
    
    (key: Key.upArrow, name: "Up Arrow"),
    (key: Key.downArrow, name: "Down Arrow"),
    (key: Key.leftArrow, name: "Left Arrow"),
    (key: Key.rightArrow, name: "Right Arrow"),
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
    (key: Key.f13, name: "F13"),
    (key: Key.f14, name: "F14"),
    (key: Key.f15, name: "F15"),
    (key: Key.f16, name: "F16"),
    (key: Key.f17, name: "F17"),
    (key: Key.f18, name: "F18"),
    (key: Key.f19, name: "F19"),
    (key: Key.f20, name: "F20"),
    
    (key: Key.volumeUp, name: "Volume Up"),
    (key: Key.volumeDown, name: "Volume Down"),
    (key: Key.mute, name: "Mute"),
    (key: Key.help, name: "Help"),
];

fileprivate struct CommandRow {
    var display: String;
    var data: [UInt8];
}

class ConfigureTapViewController: UIViewController, UIPickerViewDataSource, UIPickerViewDelegate, UITableViewDataSource {
    @IBOutlet weak var downCommands: UITableView!;
    @IBOutlet weak var upCommands: UITableView!;
    @IBOutlet weak var commandPicker: UIPickerView!;
    @IBOutlet weak var appendDown: ButtonInput!
    @IBOutlet weak var appendUp: ButtonInput!
    
    private var mouseCommand = true;
    private var downRows: [CommandRow] = [CommandRow(
        display: "Mouse down, Left",
        data: [CommandCode.mouseDown.rawValue, MouseButton.left.rawValue]
    )];
    private var upRows: [CommandRow] = [CommandRow(
        display: "Mouse up, Left",
        data: [CommandCode.mouseUp.rawValue, MouseButton.right.rawValue]
    )];

    override func viewDidLoad() {
        super.viewDidLoad();
        
        commandPicker.dataSource = self;
        commandPicker.delegate = self;
        commandPicker.reloadAllComponents();

        downCommands.dataSource = self;
        downCommands.isEditing = true;
        downCommands.reloadData();
        
        upCommands.dataSource = self;
        upCommands.isEditing = true;
        upCommands.reloadData();
    }
    
    func numberOfComponents(in pickerView: UIPickerView) -> Int {
        return 2;
    }
    
    func pickerView(_ pickerView: UIPickerView, numberOfRowsInComponent component: Int) -> Int {
        if component == 0 {
            return commandCodeRows.count;
        } else if component == 1 {
            if mouseCommand {
                return mouseButtonRows.count;
            } else {
                return keyRows.count;
            }
        } else {
            return 0;
        }
    }
    
    private func makeWhiteString(string: String) -> NSAttributedString {
        return NSAttributedString(
            string: string,
            attributes: [NSAttributedString.Key.foregroundColor: UIColor(cgColor: Colors.gray200)]
        );
    }
    
    func pickerView(_ pickerView: UIPickerView, attributedTitleForRow row: Int, forComponent component: Int) -> NSAttributedString? {
        if component == 0 {
            return makeWhiteString(string: commandCodeRows[row].name);
        } else if component == 1 {
            if mouseCommand {
                return makeWhiteString(string: mouseButtonRows[row].name);
            } else {
                return makeWhiteString(string: keyRows[row].name);
            }
        } else {
            return nil;
        }
    }
    
    func pickerView(_ pickerView: UIPickerView, didSelectRow row: Int, inComponent component: Int) {
        if component == 0 {
            if (row < 3 && !mouseCommand) || (row >= 3 && mouseCommand) {
                mouseCommand = !mouseCommand
                commandPicker.reloadComponent(1);
                commandPicker.selectRow(0, inComponent: 1, animated: false);
            }
        }
    }
    
    private func selectTableRows<T>(tableView: UITableView, _ callback: (inout [CommandRow]) -> T) -> T {
        if tableView == downCommands {
            return callback(&downRows);
        } else if tableView == upCommands {
            return callback(&upRows);
        } else {
            fatalError();
        }
    }
    
    func tableView(_ tableView: UITableView, numberOfRowsInSection section: Int) -> Int {
        return selectTableRows(tableView: tableView) { $0.count };
    }
    
    func tableView(_ tableView: UITableView, cellForRowAt indexPath: IndexPath) -> UITableViewCell {
        let cell = tableView.dequeueReusableCell(withIdentifier: "CommandCell", for: indexPath);
        cell.textLabel!.textColor = UIColor(cgColor: Colors.gray200);
        // Sets color of reorder control.
        // Might want to consider setting this for the whole app.
        cell.overrideUserInterfaceStyle = .dark;
        selectTableRows(tableView: tableView) {rows in
            cell.textLabel!.text = rows[indexPath.row].display;
        };
        return cell;
    }
    
    func tableView(_ tableView: UITableView, commit editingStyle: UITableViewCell.EditingStyle, forRowAt indexPath: IndexPath) {
        if editingStyle == .delete {
            selectTableRows(tableView: tableView) {rows in
                rows.remove(at: indexPath.row);
                return; // Suppress warning
            };
            tableView.deleteRows(at: [indexPath], with: .automatic);
        }
    }
    
    func tableView(_ tableView: UITableView, moveRowAt sourceIndexPath: IndexPath, to destinationIndexPath: IndexPath) {
        selectTableRows(tableView: tableView) {rows in
            let item = rows[sourceIndexPath.row];
            rows.remove(at: sourceIndexPath.row);
            rows.insert(item, at: destinationIndexPath.row);
        };
    }
}
