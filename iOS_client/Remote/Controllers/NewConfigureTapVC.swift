//
//  NewConfigureTapVC.swift
//  Remote
//
//  Created by Indiana Kernick on 25/6/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import UIKit

class NewConfigureTapVC: UITableViewController {
    private var downCommands: [CommandStruct] = [CommandStruct()]
    private var upCommands: [CommandStruct] = [CommandStruct()]
    
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
    }
    
    override func viewWillDisappear(_ animated: Bool) {
        super.viewWillDisappear(animated)
        navigationController?.isToolbarHidden = true
    }
    
    override func prepare(for segue: UIStoryboardSegue, sender: Any?) {
        super.prepare(for: segue, sender: sender)
        
        if let nav = segue.destination as? UINavigationController {
            let edit = nav.topViewController! as! EditCommandVC
            edit.initialize(command: nil)
            edit.updated = { [weak self] command in
                self!.upCommands.append(command)
                self!.tableView.reloadData()
            }
        } else if let edit = segue.destination as? EditCommandVC {
            let index = tableView.indexPath(for: sender as! UITableViewCell)!
            edit.initialize(command: select(index: index) { $0 })
            edit.updated = { [weak self] command in
                self!.select(index: index) { $0 = command }
                self!.tableView.reloadData()
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
