//
//  TapViewController.swift
//  Remote
//
//  Created by Indiana Kernick on 2/2/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import Foundation;

class TapViewController: BasicViewController {
    @IBOutlet weak var tap: TapInput!;
    
    private var downData = Data([CommandCode.mouseClick.rawValue, MouseButton.left.rawValue]);
    private var upData = Data();
    
    override func viewDidLoad() {
        super.viewDidLoad();
        tap.pressed = { [weak self] in
            self!.send(self!.downData);
        };
        tap.released = { [weak self] in
            self!.send(self!.upData);
        };
    }
}
