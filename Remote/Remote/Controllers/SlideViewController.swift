//
//  SlideViewController.swift
//  Remote
//
//  Created by Indiana Kernick on 1/2/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

class SlideViewController: BasicViewController {
    @IBOutlet weak var lastBtn: ButtonInput!;
    @IBOutlet weak var nextBtn: ButtonInput!;
    @IBOutlet weak var previousBtn: ButtonInput!;
    @IBOutlet weak var firstBtn: ButtonInput!;
    
    override func viewDidLoad() {
        super.viewDidLoad();
        setPressListener(for: lastBtn, with: [CommandCode.keyClick.rawValue, Key.end.rawValue]);
        setPressListener(for: nextBtn, with: [CommandCode.keyClick.rawValue, Key.rightArrow.rawValue]);
        setPressListener(for: previousBtn, with: [CommandCode.keyClick.rawValue, Key.leftArrow.rawValue]);
        setPressListener(for: firstBtn, with: [CommandCode.keyClick.rawValue, Key.home.rawValue]);
    }
}
