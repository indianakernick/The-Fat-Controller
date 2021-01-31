//
//  NumberPadViewController.swift
//  Remote
//
//  Created by Indiana Kernick on 31/1/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import UIKit;

class NumberPadViewController: UIViewController {
    private var socket = SocketManager();
    
    @IBOutlet weak var cut: ButtonInput!;
    @IBOutlet weak var copyButton: ButtonInput!;
    @IBOutlet weak var paste: ButtonInput!;
    @IBOutlet weak var delete: ButtonInput!;
    @IBOutlet weak var equal: ButtonInput!;
    @IBOutlet weak var divide: ButtonInput!;
    @IBOutlet weak var multiply: ButtonInput!;
    @IBOutlet weak var lparen: ButtonInput!;
    @IBOutlet weak var up: ButtonInput!;
    @IBOutlet weak var rparen: ButtonInput!;
    @IBOutlet weak var seven: ButtonInput!;
    @IBOutlet weak var eight: ButtonInput!;
    @IBOutlet weak var nine: ButtonInput!;
    @IBOutlet weak var subtract: ButtonInput!;
    @IBOutlet weak var left: ButtonInput!;
    @IBOutlet weak var dollar: ButtonInput!;
    @IBOutlet weak var right: ButtonInput!;
    @IBOutlet weak var four: ButtonInput!;
    @IBOutlet weak var five: ButtonInput!;
    @IBOutlet weak var six: ButtonInput!;
    @IBOutlet weak var add: ButtonInput!;
    @IBOutlet weak var less: ButtonInput!;
    @IBOutlet weak var down: ButtonInput!;
    @IBOutlet weak var greater: ButtonInput!;
    @IBOutlet weak var one: ButtonInput!;
    @IBOutlet weak var two: ButtonInput!;
    @IBOutlet weak var three: ButtonInput!;
    @IBOutlet weak var `return`: ButtonInput!;
    @IBOutlet weak var shift: ButtonInput!;
    @IBOutlet weak var caret: ButtonInput!;
    @IBOutlet weak var percent: ButtonInput!;
    @IBOutlet weak var comma: ButtonInput!;
    @IBOutlet weak var zero: ButtonInput!;
    @IBOutlet weak var period: ButtonInput!;
    
    private func setPressListener(for button: ButtonInput, with array: [UInt8]) {
        let data = Data(array);
        button.pressed = {
            self.socket.send(data);
        };
    }
    
    private func shiftPressed() {
        eight.text = "↑";
        six.text = "→";
        two.text = "↓";
        four.text = "←";
        setPressListener(for: eight, with: [CommandCode.keyClickFlags.rawValue, Key.upArrow.rawValue, Flags.shift.rawValue]);
        setPressListener(for: six, with: [CommandCode.keyClickFlags.rawValue, Key.rightArrow.rawValue, Flags.shift.rawValue]);
        setPressListener(for: two, with: [CommandCode.keyClickFlags.rawValue, Key.downArrow.rawValue, Flags.shift.rawValue]);
        setPressListener(for: four, with: [CommandCode.keyClickFlags.rawValue, Key.leftArrow.rawValue, Flags.shift.rawValue]);
    }
    
    private func shiftReleased() {
        eight.text = "8";
        six.text = "6";
        two.text = "2";
        four.text = "4";
        setPressListener(for: eight, with: [CommandCode.keyClick.rawValue, Key.n8.rawValue]);
        setPressListener(for: six, with: [CommandCode.keyClick.rawValue, Key.n6.rawValue]);
        setPressListener(for: two, with: [CommandCode.keyClick.rawValue, Key.n2.rawValue]);
        setPressListener(for: four, with: [CommandCode.keyClick.rawValue, Key.n4.rawValue]);
    }
    
    override func viewDidLoad() {
        super.viewDidLoad();
        
        socket.connect();
        
        setPressListener(for: cut, with: [CommandCode.keyClickFlags.rawValue, Key.x.rawValue, Flags.command.rawValue]);
        setPressListener(for: copyButton, with: [CommandCode.keyClickFlags.rawValue, Key.c.rawValue, Flags.command.rawValue]);
        setPressListener(for: paste, with: [CommandCode.keyClickFlags.rawValue, Key.v.rawValue, Flags.command.rawValue]);
        setPressListener(for: delete, with: [CommandCode.keyClick.rawValue, Key.delete.rawValue]);
        setPressListener(for: equal, with: [CommandCode.keyClick.rawValue, Key.equal.rawValue]);
        setPressListener(for: divide, with: [CommandCode.keyClick.rawValue, Key.slash.rawValue]);
        setPressListener(for: multiply, with: [CommandCode.keyClickFlags.rawValue, Key.n8.rawValue, Flags.shift.rawValue]);
        
        setPressListener(for: lparen, with: [CommandCode.keyClickFlags.rawValue, Key.n9.rawValue, Flags.shift.rawValue]);
        setPressListener(for: up, with: [CommandCode.keyClick.rawValue, Key.upArrow.rawValue]);
        setPressListener(for: rparen, with: [CommandCode.keyClickFlags.rawValue, Key.n0.rawValue, Flags.shift.rawValue]);
        setPressListener(for: seven, with: [CommandCode.keyClick.rawValue, Key.n7.rawValue]);
        setPressListener(for: eight, with: [CommandCode.keyClick.rawValue, Key.n8.rawValue]);
        setPressListener(for: nine, with: [CommandCode.keyClick.rawValue, Key.n9.rawValue]);
        setPressListener(for: subtract, with: [CommandCode.keyClick.rawValue, Key.minus.rawValue]);
        
        setPressListener(for: left, with: [CommandCode.keyClick.rawValue, Key.leftArrow.rawValue]);
        setPressListener(for: dollar, with: [CommandCode.keyClickFlags.rawValue, Key.n4.rawValue, Flags.shift.rawValue]);
        setPressListener(for: right, with: [CommandCode.keyClick.rawValue, Key.rightArrow.rawValue]);
        setPressListener(for: four, with: [CommandCode.keyClick.rawValue, Key.n4.rawValue]);
        setPressListener(for: five, with: [CommandCode.keyClick.rawValue, Key.n5.rawValue]);
        setPressListener(for: six, with: [CommandCode.keyClick.rawValue, Key.n6.rawValue]);
        setPressListener(for: add, with: [CommandCode.keyClickFlags.rawValue, Key.equal.rawValue, Flags.shift.rawValue]);
        
        setPressListener(for: less, with: [CommandCode.keyClickFlags.rawValue, Key.comma.rawValue, Flags.shift.rawValue]);
        setPressListener(for: down, with: [CommandCode.keyClick.rawValue, Key.downArrow.rawValue]);
        setPressListener(for: greater, with: [CommandCode.keyClickFlags.rawValue, Key.period.rawValue, Flags.shift.rawValue]);
        setPressListener(for: one, with: [CommandCode.keyClick.rawValue, Key.n1.rawValue]);
        setPressListener(for: two, with: [CommandCode.keyClick.rawValue, Key.n2.rawValue]);
        setPressListener(for: three, with: [CommandCode.keyClick.rawValue, Key.n3.rawValue]);
        setPressListener(for: `return`, with: [CommandCode.keyClick.rawValue, Key.return.rawValue]);
        
        setPressListener(for: caret, with: [CommandCode.keyClickFlags.rawValue, Key.n6.rawValue, Flags.shift.rawValue]);
        setPressListener(for: percent, with: [CommandCode.keyClickFlags.rawValue, Key.n5.rawValue, Flags.shift.rawValue]);
        setPressListener(for: comma, with: [CommandCode.keyClick.rawValue, Key.comma.rawValue]);
        setPressListener(for: zero, with: [CommandCode.keyClick.rawValue, Key.n0.rawValue]);
        setPressListener(for: period, with: [CommandCode.keyClick.rawValue, Key.period.rawValue]);
        
        shift.pressed = shiftPressed;
        shift.released = shiftReleased;
    }
}
