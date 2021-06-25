//
//  EditCommandVC.swift
//  Remote
//
//  Created by Indiana Kernick on 25/6/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import UIKit

class EditCommandVC: UITableViewController {
    private var commandCode = CommandCode.mouseClick
    
    func setCommandCode(_ code: CommandCode) {
        commandCode = code
    }
    
    // --- UITableViewController --- //
    
    override func prepare(for segue: UIStoryboardSegue, sender: Any?) {
        super.prepare(for: segue, sender: sender)
        if let dest = segue.destination as? CommandVC {
            dest.setCommandCode(commandCode)
        }
    }
    
    override func viewWillAppear(_ animated: Bool) {
        super.viewWillAppear(animated)
        (view as! UITableView).reloadData()
    }
    
    override func numberOfSections(in: UITableView) -> Int {
        commandCode == .delay ? 2 : 1
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
            return tableView.dequeueReusableCell(withIdentifier: "IntField", for: indexPath)
        }
    }
    
    override func tableView(_ tableView: UITableView, titleForHeaderInSection section: Int) -> String? {
        section == 1 ? "DELAY (MILLISECONDS)" : nil
    }
}
