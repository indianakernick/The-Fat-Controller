//
//  EditCommandVC.swift
//  Remote
//
//  Created by Indiana Kernick on 25/6/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import UIKit

fileprivate enum ParameterType {
    case uint, int
}

fileprivate struct Parameter {
    let name: String?
    let type: ParameterType
}

fileprivate let parameters: [[Parameter]] = [
    [Parameter(name: "DELAY (MILLISECONDS)", type: .uint)]
]

class EditCommandVC: UITableViewController, PickerDelegate {
    private var commandCode = CommandCode.mouseClick
    
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
            dest.setValue(commandCode)
            dest.setDelegate(self)
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
            let cell = tableView.dequeueReusableCell(withIdentifier: "NumberInputCell", for: indexPath) as! NumberInputCell
            cell.numberInput.setIndent(tableView.separatorInset.left)
            // getParameter(index: indexPath.section - 1).type
            return cell
        }
    }
    
    override func tableView(_ tableView: UITableView, titleForHeaderInSection section: Int) -> String? {
        section > 0 ? getParameter(index: section - 1).name : nil
    }
    
    // --- PickerDelegate --- //
    
    func didUpdate(value: UInt8) {
        commandCode = CommandCode(rawValue: value)!
    }
}
