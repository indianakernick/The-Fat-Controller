//
//  EditCommandVC.swift
//  Remote
//
//  Created by Indiana Kernick on 25/6/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import UIKit

// There's a constraints error when changing focus between the text fields when
// there are multiple text fields. Haven't seen this happen for a while
// though so maybe it sorted itself out?

// Using a subset of the available command codes. The ASCII commands aren't
// really necessary.
fileprivate let commandCodes: [CommandCode] = [
    .delay,
    .keyDown, .keyUp, .keyClick,
    .mouseMoveRel, .mouseMoveAbs, .mouseScroll, .mouseDown, .mouseUp, .mouseClick,
    .unicodeCharDown, .unicodeCharUp, .unicodeChar, .unicodeString
]

fileprivate func enumNames<C: Collection>(values: C) -> [String]
    where C.Element: CustomStringConvertible
{
    var names: [String] = []
    names.reserveCapacity(values.count)
    for value in values {
        names.append(value.description)
    }
    return names
}

fileprivate let commandNames: [String] = enumNames(values: commandCodes)
fileprivate let keyNames: [String] = enumNames(values: Key.allCases)
fileprivate let mouseButtonNames: [String] = enumNames(values: MouseButton.allCases)

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

class EditCommandVC: UITableViewController {
    @IBAction func cancelPressed(_ sender: Any) {
        if createMode {
            dismiss(animated: true, completion: nil)
        } else {
            navigationController?.popViewController(animated: true)
        }
    }
    
    @IBAction func donePressed(_ sender: Any) {
        command.normalize()
        updated(command)
        if createMode {
            dismiss(animated: true, completion: nil)
        } else {
            navigationController?.popViewController(animated: true)
        }
    }
    
    private var command = CommandStruct()
    private var createMode = false
    
    private func getIndex() -> Int {
        commandCodes.firstIndex(of: command.code)!
    }
    
    private func setIndex(_ index: Int) {
        command.code = commandCodes[index]
    }
    
    private func getTextInputCell(mode: TextInputCell.Mode, indexPath: IndexPath) -> TextInputCell {
        let cell = tableView.dequeueReusableCell(withIdentifier: "TextInput", for: indexPath) as! TextInputCell
        cell.setMode(mode)
        return cell
    }
    
    // --- EditCommandVC --- //
    
    var updated: (CommandStruct) -> Void = { command in }
    
    func initialize(command: CommandStruct?) {
        if command == nil {
            self.command = CommandStruct()
            createMode = true
            title = "Create Command"
        } else {
            self.command = command!
            createMode = false
            title = "Edit Command"
        }
    }
    
    // --- UIViewController --- //

    override func viewWillAppear(_ animated: Bool) {
        super.viewWillAppear(animated)
        // Not sure where else to put this assertion...
        assert(commandCodes.count == parameterNames.count)
        tableView.reloadData()
    }
    
    override func prepare(for segue: UIStoryboardSegue, sender: Any?) {
        super.prepare(for: segue, sender: sender)
        
        guard let dest = segue.destination as? PickerVC else { return }
        guard let cell = sender as? UITableViewCell else { return }
        
        if cell.reuseIdentifier == "Command" {
            dest.initialize(name: "Command", value: getIndex(), cases: commandNames)
            dest.updated = { [weak self] value in
                self!.setIndex(value)
            }
            return
        }
        
        switch command.code {
        case .keyDown, .keyUp, .keyClick:
            assert(cell.reuseIdentifier == "Key")
            dest.initialize(name: "Key", value: Int(command.key.rawValue), cases: keyNames)
            dest.updated = { [weak self] value in
                self!.command.key = Key(rawValue: UInt8(value))!
            }
        case .mouseDown, .mouseUp, .mouseClick:
            assert(cell.reuseIdentifier == "MouseButton")
            dest.initialize(name: "Mouse Button", value: Int(command.button.rawValue), cases: mouseButtonNames)
            dest.updated = { [weak self] value in
                self!.command.button = MouseButton(rawValue: UInt8(value))!
            }
        default:
            assert(false)
        }
    }
    
    // --- UITableViewController --- //
    
    override func numberOfSections(in: UITableView) -> Int {
        1 + parameterNames[Int(getIndex())].count
    }
    
    override func tableView(_ tableView: UITableView, numberOfRowsInSection section: Int) -> Int {
        1
    }
    
    override func tableView(_ tableView: UITableView, cellForRowAt indexPath: IndexPath) -> UITableViewCell {
        if indexPath.section == 0 {
            let cell = tableView.dequeueReusableCell(withIdentifier: "Command", for: indexPath)
            cell.detailTextLabel!.text = commandNames[Int(getIndex())]
            return cell
        }
        
        let index = indexPath.section - 1
        
        switch command.code {
        case .delay:
            assert(index == 0)
            let cell = getTextInputCell(mode: .uint, indexPath: indexPath)
            cell.textInput.text = String(command.delay)
            cell.textChanged = { [weak self] text in
                self!.command.delay = UInt16(text)!
            }
            return cell
        
        case .keyDown, .keyUp, .keyClick:
            assert(index == 0)
            let cell = tableView.dequeueReusableCell(withIdentifier: "Key", for: indexPath)
            cell.detailTextLabel!.text = command.key.description
            return cell
        
        case .mouseMoveRel, .mouseMoveAbs, .mouseScroll:
            assert(index == 0 || index == 1)
            let cell = getTextInputCell(mode: .int, indexPath: indexPath)
            cell.textInput.text = String(index == 0 ? command.x : command.y)
            cell.textChanged = { [weak self] text in
                if index == 0 {
                    self!.command.x = Int16(text)!
                } else {
                    self!.command.y = Int16(text)!
                }
            }
            return cell
        
        case .mouseDown, .mouseUp, .mouseClick:
            assert(index == 0)
            let cell = tableView.dequeueReusableCell(withIdentifier: "MouseButton", for: indexPath)
            cell.detailTextLabel!.text = command.button.description
            return cell
        
        case .unicodeCharDown, .unicodeCharUp, .unicodeChar:
            assert(index == 0)
            let cell = getTextInputCell(mode: .char, indexPath: indexPath)
            cell.textInput.text = String(command.char)
            cell.textChanged = { [weak self] text in
                let scalars = text.unicodeScalars
                assert(scalars.count == 1)
                self!.command.char = scalars.first!
            }
            return cell
            
        case .unicodeString:
            assert(index == 0)
            let cell = tableView.dequeueReusableCell(withIdentifier: "MultilineTextInput", for: indexPath) as! MultilineTextInputCell
            cell.textInput.text = command.string
            cell.textChanged = { [weak self] text in
                self!.command.string = text
            }
            return cell
        
        default:
            assert(false)
        }
    }
    
    override func tableView(_ tableView: UITableView, titleForHeaderInSection section: Int) -> String? {
        section > 0 ? parameterNames[Int(getIndex())][section - 1] : nil
    }
}
