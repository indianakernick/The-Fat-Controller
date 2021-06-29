//
//  ConfigureTapVC.swift
//  Remote
//
//  Created by Indiana Kernick on 25/6/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import UIKit

fileprivate func commandsFromPlist(_ plist: [Any]) -> [CommandStruct]? {
    var commands: [CommandStruct] = []
    
    // Maybe it would be easier to just store the bytes (CommandData)? Although,
    // the current approach is pretty simple and the error checking is easy.
    
    for item in plist {
        guard let dict = item as? [String: Any] else { return nil }
        guard let rawCode = dict["command"] as? UInt8 else { return nil }
        guard let code = CommandCode(rawValue: rawCode) else { return nil }
        guard let rawKey = dict["key"] as? UInt8 else { return nil }
        guard let key = Key(rawValue: rawKey) else { return nil }
        guard let rawButton = dict["mouseButton"] as? UInt8 else { return nil }
        guard let button = MouseButton(rawValue: rawButton) else { return nil }
        guard let delay = dict["delay"] as? UInt16 else { return nil }
        guard let x = dict["x"] as? Int16 else { return nil }
        guard let y = dict["y"] as? Int16 else { return nil }
        guard let rawChar = dict["char"] as? UInt32 else { return nil }
        guard let char = Unicode.Scalar(rawChar) else { return nil }
        guard let string = dict["string"] as? String else { return nil }
        
        var command = CommandStruct()
        command.code = code
        command.key = key
        command.button = button
        command.delay = delay
        command.x = x
        command.y = y
        command.char = char
        command.string = string
        command.normalize()
        commands.append(command)
    }
    
    return commands
}

fileprivate func commandsToPlist(_ commands: [CommandStruct]) -> [Any] {
    var plist: [[String: Any]] = []
    
    for command in commands {
        var dict: [String: Any] = [:]
        dict["command"] = command.code.rawValue
        dict["key"] = command.key.rawValue
        dict["mouseButton"] = command.button.rawValue
        dict["delay"] = command.delay
        dict["x"] = command.x
        dict["y"] = command.y
        dict["char"] = command.char.value
        dict["string"] = command.string
        dict["data"] = command.data
        plist.append(dict)
    }
    
    return plist
}

class ConfigureTapVC: UITableViewController {
    static let defaultDown: CommandStruct = {
        var command = CommandStruct()
        command.code = .mouseDown
        return command
    }()
    static let defaultUp: CommandStruct = {
        var command = CommandStruct()
        command.code = .mouseUp
        return command
    }()
    
    private var downCommands: [CommandStruct] = []
    private var upCommands: [CommandStruct] = []
    
    @objc private func addButtonPressed() {
        performSegue(withIdentifier: "CreateCommand", sender: self)
    }

    private func select<R>(index: IndexPath, get: (inout CommandStruct) -> R) -> R {
        if index.section == 0 {
            return get(&downCommands[index.row])
        } else {
            return get(&upCommands[index.row])
        }
    }

    private func selectList(section: Int, get: (inout [CommandStruct]) -> Void) {
        if section == 0 {
            get(&downCommands)
        } else {
            get(&upCommands)
        }
    }
    
    private func load() {
        if
            let downPlist = Storage.getTapDownCommandList(),
            let upPlist = Storage.getTapUpCommandList(),
            let downCommands = commandsFromPlist(downPlist),
            let upCommands = commandsFromPlist(upPlist)
        {
            self.downCommands = downCommands
            self.upCommands = upCommands
        } else {
            downCommands = [ConfigureTapVC.defaultDown]
            upCommands = [ConfigureTapVC.defaultUp]
        }
        save()
        tableView.reloadData()
    }
    
    private func save() {
        Storage.setTapDownCommandList(commandsToPlist(downCommands))
        Storage.setTapUpCommandList(commandsToPlist(upCommands))
    }
    
    // --- UIViewController --- //
    
    override func viewDidLoad() {
        super.viewDidLoad()
        navigationItem.rightBarButtonItem = editButtonItem
        setToolbarItems([
            UIBarButtonItem.flexibleSpace(),
            UIBarButtonItem(barButtonSystemItem: .add, target: self, action: #selector(addButtonPressed)),
            UIBarButtonItem.flexibleSpace()
        ], animated: true)
        navigationController?.toolbar.isTranslucent = false
        navigationController?.toolbar.barTintColor = UIColor(cgColor: Colors.gray900)
    }
    
    override func viewWillAppear(_ animated: Bool) {
        super.viewWillAppear(animated)
        navigationController?.isToolbarHidden = false
        load()
        tableView.reloadData()
    }
    
    override func viewWillDisappear(_ animated: Bool) {
        super.viewWillDisappear(animated)
        navigationController?.isToolbarHidden = true
        save()
        TapVC.instance?.updateData()
    }
    
    override func prepare(for segue: UIStoryboardSegue, sender: Any?) {
        super.prepare(for: segue, sender: sender)
        
        if let nav = segue.destination as? UINavigationController {
            let edit = nav.topViewController! as! EditCommandVC
            edit.initialize(command: nil)
            edit.updated = { [weak self] command in
                self!.upCommands.append(command)
                self!.tableView.reloadData()
                self!.save()
            }
        } else if let edit = segue.destination as? EditCommandVC {
            let index = tableView.indexPath(for: sender as! UITableViewCell)!
            edit.initialize(command: select(index: index) { $0 })
            edit.updated = { [weak self] command in
                self!.select(index: index) { $0 = command }
                self!.tableView.reloadData()
                self!.save()
            }
        }
    }
    
    // --- UITableViewController --- //
    
    override func numberOfSections(in tableView: UITableView) -> Int {
        2
    }
    
    override func tableView(_ tableView: UITableView, numberOfRowsInSection section: Int) -> Int {
        section == 0 ? downCommands.count : upCommands.count
    }
    
    override func tableView(_ tableView: UITableView, titleForHeaderInSection section: Int) -> String? {
        ["Down Commands", "Up Commands"][section]
    }
    
    override func tableView(_ tableView: UITableView, willDisplayHeaderView view: UIView, forSection section: Int) {
        view.tintColor = UIColor(cgColor: Colors.gray800)
    }
    
    override func tableView(_ tableView: UITableView, cellForRowAt indexPath: IndexPath) -> UITableViewCell {
        let cell = tableView.dequeueReusableCell(withIdentifier: "CommandCell", for: indexPath)
        let command = select(index: indexPath) { $0 }
        cell.textLabel!.text = command.code.description
        cell.detailTextLabel!.text = command.parameterDescription
        return cell
    }
    
    override func tableView(_ tableView: UITableView, commit editingStyle: UITableViewCell.EditingStyle, forRowAt indexPath: IndexPath) {
        if editingStyle == .delete {
            selectList(section: indexPath.section) {
                $0.remove(at: indexPath.row)
            }
            tableView.deleteRows(at: [indexPath], with: .fade)
        }
    }
    
    override func tableView(_ tableView: UITableView, moveRowAt sourceIndexPath: IndexPath, to destinationIndexPath: IndexPath) {
        var command: CommandStruct! = nil
        
        selectList(section: sourceIndexPath.section) {
            command = $0[sourceIndexPath.row]
            $0.remove(at: sourceIndexPath.row)
        }
        
        selectList(section: destinationIndexPath.section) {
            $0.insert(command, at: destinationIndexPath.row)
        }
    }
}
