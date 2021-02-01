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
    @IBOutlet weak var volume: SimpleVolumeInput!;
    
    override func viewDidLoad() {
        super.viewDidLoad();
        
        let nextListener = makeListener(with: [CommandCode.keyClick.rawValue, Key.rightArrow.rawValue]);
        let previousListener = makeListener(with: [CommandCode.keyClick.rawValue, Key.leftArrow.rawValue]);
        
        nextButton.pressed = nextListener;
        previous.pressed = previousListener;
        volume.upPressed = nextListener;
        volume.downPressed = previousListener;
        
        setPressListener(for: last, with: [CommandCode.keyClick.rawValue, Key.end.rawValue]);
        setPressListener(for: first, with: [CommandCode.keyClick.rawValue, Key.home.rawValue]);
    }
}
