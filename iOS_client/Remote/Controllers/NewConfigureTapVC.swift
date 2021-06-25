//
//  NewConfigureTapVC.swift
//  Remote
//
//  Created by Indiana Kernick on 25/6/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import UIKit

class NewConfigureTapVC: UITableViewController {
    @IBAction func editPressed(_ sender: Any) {
        isEditing = true
    }
    
    override func viewDidLoad() {
        super.viewDidLoad()
    }
    
    override func numberOfSections(in tableView: UITableView) -> Int {
        2
    }
    
    override func tableView(_ tableView: UITableView, numberOfRowsInSection section: Int) -> Int {
        1
    }
    
    override func tableView(_ tableView: UITableView, titleForHeaderInSection section: Int) -> String? {
        ["Down Commands", "Up Commands"][section]
    }
    
    override func tableView(_ tableView: UITableView, cellForRowAt indexPath: IndexPath) -> UITableViewCell {
        let cell = tableView.dequeueReusableCell(withIdentifier: "CommandCell", for: indexPath)
        cell.textLabel!.text = "Mouse Move Relative" // rows[indexPath.row].display
        cell.detailTextLabel!.text = "-32000, -32000"
        return cell
    }
    
    override func tableView(_ tableView: UITableView, commit editingStyle: UITableViewCell.EditingStyle, forRowAt indexPath: IndexPath) {
        if editingStyle == .delete {
            // rows.remove(at: indexPath.row)
            tableView.deleteRows(at: [indexPath], with: .fade)
        }
    }
    
    override func tableView(_ tableView: UITableView, moveRowAt sourceIndexPath: IndexPath, to destinationIndexPath: IndexPath) {
        //let item = rows[sourceIndexPath.row]
        //rows.remove(at: sourceIndexPath.row)
        //rows.insert(item, at: destinationIndexPath.row)
    }
}
