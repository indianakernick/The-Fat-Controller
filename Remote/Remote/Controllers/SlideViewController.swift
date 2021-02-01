//
//  SlideViewController.swift
//  Remote
//
//  Created by Indiana Kernick on 1/2/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

class SlideViewController: BasicViewController {
    @IBOutlet weak var last: ButtonInput!;
    @IBOutlet weak var nextButton: ButtonInput!;
    @IBOutlet weak var previous: ButtonInput!;
    @IBOutlet weak var first: ButtonInput!;
    
    override func viewDidLoad() {
        super.viewDidLoad();
        
        setPressListener(for: last, with: [CommandCode.keyClick.rawValue, Key.end.rawValue]);
        setPressListener(for: nextButton, with: [CommandCode.keyClick.rawValue, Key.rightArrow.rawValue]);
        setPressListener(for: previous, with: [CommandCode.keyClick.rawValue, Key.leftArrow.rawValue]);
        setPressListener(for: first, with: [CommandCode.keyClick.rawValue, Key.home.rawValue]);
    }
}
