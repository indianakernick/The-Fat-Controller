//
//  NumberPadViewController.swift
//  Remote
//
//  Created by Indiana Kernick on 31/1/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

class NumberPadViewController: BasicViewController {
    @IBOutlet weak var cutBtn: ButtonInput!
    @IBOutlet weak var copyBtn: ButtonInput!
    @IBOutlet weak var pasteBtn: ButtonInput!
    @IBOutlet weak var equalBtn: ButtonInput!
    @IBOutlet weak var divideBtn: ButtonInput!
    @IBOutlet weak var multiplyBtn: ButtonInput!
    @IBOutlet weak var deleteBtn: ButtonInput!
    @IBOutlet weak var lparenBtn: ButtonInput!
    @IBOutlet weak var upBtn: ButtonInput!
    @IBOutlet weak var rparenBtn: ButtonInput!
    @IBOutlet weak var sevenBtn: ButtonInput!
    @IBOutlet weak var eightBtn: ButtonInput!
    @IBOutlet weak var nineBtn: ButtonInput!
    @IBOutlet weak var subtractBtn: ButtonInput!
    @IBOutlet weak var leftBtn: ButtonInput!
    @IBOutlet weak var dollarBtn: ButtonInput!
    @IBOutlet weak var rightBtn: ButtonInput!
    @IBOutlet weak var fourBtn: ButtonInput!
    @IBOutlet weak var fiveBtn: ButtonInput!
    @IBOutlet weak var sixBtn: ButtonInput!
    @IBOutlet weak var addBtn: ButtonInput!
    @IBOutlet weak var lessBtn: ButtonInput!
    @IBOutlet weak var downBtn: ButtonInput!
    @IBOutlet weak var greaterBtn: ButtonInput!
    @IBOutlet weak var oneBtn: ButtonInput!
    @IBOutlet weak var twoBtn: ButtonInput!
    @IBOutlet weak var threeBtn: ButtonInput!
    @IBOutlet weak var returnBtn: ButtonInput!
    @IBOutlet weak var shiftBtn: ButtonInput!
    @IBOutlet weak var caretBtn: ButtonInput!
    @IBOutlet weak var percentBtn: ButtonInput!
    @IBOutlet weak var commaBtn: ButtonInput!
    @IBOutlet weak var zeroBtn: ButtonInput!
    @IBOutlet weak var periodBtn: ButtonInput!
    
    private var pressShift = {}
    private var releaseShift = {}
    
    private func shiftPressed() {
        eightBtn.text = "↑"
        sixBtn.text = "→"
        twoBtn.text = "↓"
        fourBtn.text = "←"
        setPressListener(for: eightBtn, with: [CommandCode.keyClickFlags.rawValue, Key.upArrow.rawValue, Flags.shift.rawValue])
        setPressListener(for: sixBtn, with: [CommandCode.keyClickFlags.rawValue, Key.rightArrow.rawValue, Flags.shift.rawValue])
        setPressListener(for: twoBtn, with: [CommandCode.keyClickFlags.rawValue, Key.downArrow.rawValue, Flags.shift.rawValue])
        setPressListener(for: fourBtn, with: [CommandCode.keyClickFlags.rawValue, Key.leftArrow.rawValue, Flags.shift.rawValue])
        pressShift()
    }
    
    private func shiftReleased() {
        eightBtn.text = "8"
        sixBtn.text = "6"
        twoBtn.text = "2"
        fourBtn.text = "4"
        setPressListener(for: eightBtn, with: [CommandCode.keyClick.rawValue, Key.n8.rawValue])
        setPressListener(for: sixBtn, with: [CommandCode.keyClick.rawValue, Key.n6.rawValue])
        setPressListener(for: twoBtn, with: [CommandCode.keyClick.rawValue, Key.n2.rawValue])
        setPressListener(for: fourBtn, with: [CommandCode.keyClick.rawValue, Key.n4.rawValue])
        releaseShift()
    }
    
    override func viewDidLoad() {
        super.viewDidLoad()
        
        pressShift = makeListener(with: [CommandCode.keyDown.rawValue, Key.shift.rawValue])
        releaseShift = makeListener(with: [CommandCode.keyUp.rawValue, Key.shift.rawValue])
        
        setPressListener(for: cutBtn, with: [CommandCode.keyClickFlags.rawValue, Key.x.rawValue, Flags.command.rawValue])
        setPressListener(for: copyBtn, with: [CommandCode.keyClickFlags.rawValue, Key.c.rawValue, Flags.command.rawValue])
        setPressListener(for: pasteBtn, with: [CommandCode.keyClickFlags.rawValue, Key.v.rawValue, Flags.command.rawValue])
        setPressListener(for: equalBtn, with: [CommandCode.keyClick.rawValue, Key.equal.rawValue])
        setPressListener(for: divideBtn, with: [CommandCode.keyClick.rawValue, Key.slash.rawValue])
        setPressListener(for: multiplyBtn, with: [CommandCode.keyClickFlags.rawValue, Key.n8.rawValue, Flags.shift.rawValue])
        setPressListener(for: deleteBtn, with: [CommandCode.keyClick.rawValue, Key.delete.rawValue])
        
        setPressListener(for: lparenBtn, with: [CommandCode.keyClickFlags.rawValue, Key.n9.rawValue, Flags.shift.rawValue])
        setPressListener(for: upBtn, with: [CommandCode.keyClick.rawValue, Key.upArrow.rawValue])
        setPressListener(for: rparenBtn, with: [CommandCode.keyClickFlags.rawValue, Key.n0.rawValue, Flags.shift.rawValue])
        setPressListener(for: sevenBtn, with: [CommandCode.keyClick.rawValue, Key.n7.rawValue])
        setPressListener(for: eightBtn, with: [CommandCode.keyClick.rawValue, Key.n8.rawValue])
        setPressListener(for: nineBtn, with: [CommandCode.keyClick.rawValue, Key.n9.rawValue])
        setPressListener(for: subtractBtn, with: [CommandCode.keyClick.rawValue, Key.minus.rawValue])
        
        setPressListener(for: leftBtn, with: [CommandCode.keyClick.rawValue, Key.leftArrow.rawValue])
        setPressListener(for: dollarBtn, with: [CommandCode.keyClickFlags.rawValue, Key.n4.rawValue, Flags.shift.rawValue])
        setPressListener(for: rightBtn, with: [CommandCode.keyClick.rawValue, Key.rightArrow.rawValue])
        setPressListener(for: fourBtn, with: [CommandCode.keyClick.rawValue, Key.n4.rawValue])
        setPressListener(for: fiveBtn, with: [CommandCode.keyClick.rawValue, Key.n5.rawValue])
        setPressListener(for: sixBtn, with: [CommandCode.keyClick.rawValue, Key.n6.rawValue])
        setPressListener(for: addBtn, with: [CommandCode.keyClickFlags.rawValue, Key.equal.rawValue, Flags.shift.rawValue])
        
        setPressListener(for: lessBtn, with: [CommandCode.keyClickFlags.rawValue, Key.comma.rawValue, Flags.shift.rawValue])
        setPressListener(for: downBtn, with: [CommandCode.keyClick.rawValue, Key.downArrow.rawValue])
        setPressListener(for: greaterBtn, with: [CommandCode.keyClickFlags.rawValue, Key.period.rawValue, Flags.shift.rawValue])
        setPressListener(for: oneBtn, with: [CommandCode.keyClick.rawValue, Key.n1.rawValue])
        setPressListener(for: twoBtn, with: [CommandCode.keyClick.rawValue, Key.n2.rawValue])
        setPressListener(for: threeBtn, with: [CommandCode.keyClick.rawValue, Key.n3.rawValue])
        setPressListener(for: returnBtn, with: [CommandCode.keyClick.rawValue, Key.return.rawValue])
        
        setPressListener(for: caretBtn, with: [CommandCode.keyClickFlags.rawValue, Key.n6.rawValue, Flags.shift.rawValue])
        setPressListener(for: percentBtn, with: [CommandCode.keyClickFlags.rawValue, Key.n5.rawValue, Flags.shift.rawValue])
        setPressListener(for: commaBtn, with: [CommandCode.keyClick.rawValue, Key.comma.rawValue])
        setPressListener(for: zeroBtn, with: [CommandCode.keyClick.rawValue, Key.n0.rawValue])
        setPressListener(for: periodBtn, with: [CommandCode.keyClick.rawValue, Key.period.rawValue])
        
        shiftBtn.pressed = shiftPressed
        shiftBtn.released = shiftReleased
    }
}
