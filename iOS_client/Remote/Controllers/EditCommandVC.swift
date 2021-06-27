//
//  EditCommandVC.swift
//  Remote
//
//  Created by Indiana Kernick on 25/6/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import UIKit
 
enum CommandParam {
    case delay(UInt16)
    case keyDown(Key)
    case keyUp(Key)
    case keyClick(Key)
    case mouseMoveRel(Int16, Int16)
    case mouseMoveAbs(Int16, Int16)
    case mouseScroll(Int16, Int16)
    case mouseDown(MouseButton)
    case mouseUp(MouseButton)
    case mouseClick(MouseButton)
    case unicodeCharDown(Unicode.Scalar)
    case unicodeCharUp(Unicode.Scalar)
    case unicodeChar(Unicode.Scalar)
    case unicodeString(String)
}

fileprivate let commandNames: [String] = [
    CommandCode.delay.description,
    CommandCode.keyDown.description,
    CommandCode.keyUp.description,
    CommandCode.keyClick.description,
    CommandCode.mouseMoveRel.description,
    CommandCode.mouseMoveAbs.description,
    CommandCode.mouseScroll.description,
    CommandCode.mouseDown.description,
    CommandCode.mouseUp.description,
    CommandCode.mouseClick.description,
    CommandCode.unicodeCharDown.description,
    CommandCode.unicodeCharUp.description,
    CommandCode.unicodeChar.description,
    CommandCode.unicodeString.description,
]

fileprivate func enumNames<E: Enum>(_ e: E) -> [String] {
    var names: [String] = []
    names.reserveCapacity(E.allCases.count)
    for key in E.allCases {
        names.append(key.description)
    }
    return names
}

fileprivate let keyNames: [String] = enumNames(Key(rawValue: 0)!)
fileprivate let mouseButtonNames: [String] = enumNames(MouseButton(rawValue: 0)!)

fileprivate let commandDefaults: [CommandParam] = [
    .delay(0),
    .keyDown(.space),
    .keyUp(.space),
    .keyClick(.space),
    .mouseMoveRel(0, 0),
    .mouseMoveAbs(0, 0),
    .mouseScroll(0, 0),
    .mouseDown(.left),
    .mouseUp(.left),
    .mouseClick(.left),
    .unicodeCharDown("a"),
    .unicodeCharUp("a"),
    .unicodeChar("a"),
    .unicodeString(""),
]

// there's a constraints error when changing focus between the text fields when
// there are multiple text fields

// should mousemoveabs use unsigned integers?

fileprivate let parameterNames: [[String?]] = [
    ["DELAY (MILLISECONDS)"],
    [nil],
    [nil],
    [nil],
    ["X (PIXELS)", "Y (PIXELS)"],
    ["X (PIXELS)", "Y (PIXELS)"],
    ["X (PIXELS)", "Y (PIXELS)"],
    [nil],
    [nil],
    [nil],
    ["CHARACTER"],
    ["CHARACTER"],
    ["CHARACTER"],
    ["STRING"],
]

// I was trying to avoid having a bunch of command switch statements everywhere
// but doing this any other way was just turning into a horrible mess.

class EditCommandVC: UITableViewController {
    private var commandCode: UInt8 = 0
    private var command = commandDefaults[0]
    
    // --- EditCommandVC --- //
    
    var updated: (CommandParam) -> Void = { command in }
    
    func initialize(command: CommandParam) {
        self.command = command
        commandCode = {
            switch command {
            case .delay: return 0
            case .keyDown: return 1
            case .keyUp: return 2
            case .keyClick: return 3
            case .mouseMoveRel: return 4
            case .mouseMoveAbs: return 5
            case .mouseScroll: return 6
            case .mouseDown: return 7
            case .mouseUp: return 8
            case .mouseClick: return 9
            case .unicodeCharDown: return 10
            case .unicodeCharUp: return 11
            case .unicodeChar: return 12
            case .unicodeString: return 13
            }
        }()
    }
    
    // --- UITableViewController --- //
    
    // Need to use viewDidDisappear to run after textFieldDidEndEditing.
    // That seems problematic though
    override func viewDidDisappear(_ animated: Bool) {
        super.viewDidDisappear(animated)
        if isMovingFromParent {
            print(command)
            updated(command)
        }
    }
    
    override func prepare(for segue: UIStoryboardSegue, sender: Any?) {
        super.prepare(for: segue, sender: sender)
        if let dest = segue.destination as? PickerVC {
            if let cell = sender as? UITableViewCell {
                if cell.reuseIdentifier == "Command" {
                    dest.initialize(name: "Command", value: commandCode, cases: commandNames)
                    dest.updated = { [weak self] value in
                        if self!.commandCode != value {
                            self!.commandCode = value
                            self!.command = commandDefaults[Int(value)]
                        }
                    }
                    return
                }
                
                switch command {
                case .keyDown(let key), .keyUp(let key), .keyClick(let key):
                    assert(cell.reuseIdentifier == "Key")
                    dest.initialize(name: "Key", value: key.rawValue, cases: keyNames)
                    dest.updated = { [weak self] value in
                        let key = Key(rawValue: value)!
                        self!.command = {
                            switch self!.command {
                            case .keyDown: return .keyDown(key)
                            case .keyUp: return .keyUp(key)
                            case .keyClick: return .keyClick(key)
                            default: assert(false)
                            }
                        }()
                    }
                    break
                case .mouseDown(let button), .mouseUp(let button), .mouseClick(let button):
                    assert(cell.reuseIdentifier == "MouseButton")
                    dest.initialize(name: "Mouse Button", value: button.rawValue, cases: mouseButtonNames)
                    dest.updated = { [weak self] value in
                        let button = MouseButton(rawValue: value)!
                        self!.command = {
                            switch self!.command {
                            case .mouseDown: return .mouseDown(button)
                            case .mouseUp: return .mouseUp(button)
                            case .mouseClick: return .mouseClick(button)
                            default: assert(false)
                            }
                        }()
                    }
                    break
                default:
                    assert(false)
                }
            }
        }
    }
    
    override func viewWillAppear(_ animated: Bool) {
        super.viewWillAppear(animated)
        tableView.reloadData()
    }
    
    override func numberOfSections(in: UITableView) -> Int {
        1 + parameterNames[Int(commandCode)].count
    }
    
    override func tableView(_ tableView: UITableView, numberOfRowsInSection section: Int) -> Int {
        1
    }
    
    private func getTextInputCell(mode: TextInput.Mode, indexPath: IndexPath) -> TextInputCell {
        let cell = tableView.dequeueReusableCell(withIdentifier: "TextInput", for: indexPath) as! TextInputCell
        cell.textInput.setIndent(tableView.separatorInset.left)
        cell.textInput.setMode(mode)
        return cell
    }
    
    override func tableView(_ tableView: UITableView, cellForRowAt indexPath: IndexPath) -> UITableViewCell {
        if indexPath.section == 0 {
            let cell = tableView.dequeueReusableCell(withIdentifier: "Command", for: indexPath)
            cell.detailTextLabel!.text = commandNames[Int(commandCode)]
            return cell
        } else {
            let index = indexPath.section - 1
            
            switch command {
            case .delay(let delay):
                assert(index == 0)
                let cell = getTextInputCell(mode: .uint, indexPath: indexPath)
                cell.textInput.text = String(delay)
                cell.textInput.textChanged = { [weak self] text in
                    self!.command = .delay(UInt16(text)!)
                }
                return cell
            case .keyDown(let key), .keyUp(let key), .keyClick(let key):
                assert(index == 0)
                let cell = tableView.dequeueReusableCell(withIdentifier: "Key", for: indexPath)
                cell.detailTextLabel!.text = key.description
                return cell
            case .mouseMoveRel(let x, let y), .mouseMoveAbs(let x, let y), .mouseScroll(let x, let y):
                assert(index == 0 || index == 1)
                let cell = getTextInputCell(mode: .int, indexPath: indexPath)
                cell.textInput.text = String(index == 0 ? x : y)
                cell.textInput.textChanged = { [weak self] text in
                    // maybe the struct would be better after all?
                    switch self!.command {
                    case .mouseMoveRel(var x, var y):
                        if index == 0 {
                            x = Int16(text)!
                        } else {
                            y = Int16(text)!
                        }
                        self!.command = .mouseMoveRel(x, y)
                        break
                    case .mouseMoveAbs(var x, var y):
                        if index == 0 {
                            x = Int16(text)!
                        } else {
                            y = Int16(text)!
                        }
                        self!.command = .mouseMoveAbs(x, y)
                        break
                    case .mouseScroll(var x, var y):
                        if index == 0 {
                            x = Int16(text)!
                        } else {
                            y = Int16(text)!
                        }
                        self!.command = .mouseScroll(x, y)
                        break
                    default:
                        assert(false)
                    }
                }
                return cell
            case .mouseDown(let button), .mouseUp(let button), .mouseClick(let button):
                assert(index == 0)
                let cell = tableView.dequeueReusableCell(withIdentifier: "MouseButton", for: indexPath)
                cell.detailTextLabel!.text = button.description
                return cell
            case .unicodeCharDown(let char), .unicodeCharUp(let char), .unicodeChar(let char):
                assert(index == 0)
                let cell = getTextInputCell(mode: .char, indexPath: indexPath)
                cell.textInput.text = String(char)
                cell.textInput.textChanged = { [weak self] text in
                    let scalars = text.unicodeScalars
                    assert(scalars.count == 1)
                    let char = scalars.first!
                    self!.command = {
                        switch self!.command {
                        case .unicodeCharDown: return .unicodeCharDown(char);
                        case .unicodeCharUp: return .unicodeCharUp(char);
                        case .unicodeChar: return .unicodeChar(char);
                        default: assert(false)
                        }
                    }()
                }
                return cell
            case .unicodeString(let string):
                assert(index == 0)
                let cell = getTextInputCell(mode: .string, indexPath: indexPath)
                cell.textInput.text = string
                cell.textInput.textChanged = { [weak self] text in
                    self!.command = .unicodeString(text)
                }
                return cell
            }
        }
    }
    
    override func tableView(_ tableView: UITableView, titleForHeaderInSection section: Int) -> String? {
        section > 0 ? parameterNames[Int(commandCode)][section - 1] : nil
    }
}
