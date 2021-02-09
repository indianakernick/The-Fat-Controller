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
        setPressListener(for: eightBtn, with: Command.keyClick(Key.upArrow, with: Key.shift))
        setPressListener(for: sixBtn, with: Command.keyClick(Key.rightArrow, with: Key.shift))
        setPressListener(for: twoBtn, with: Command.keyClick(Key.downArrow, with: Key.shift))
        setPressListener(for: fourBtn, with: Command.keyClick(Key.leftArrow, with: Key.shift))
        pressShift()
    }
    
    private func shiftReleased() {
        eightBtn.text = "8"
        sixBtn.text = "6"
        twoBtn.text = "2"
        fourBtn.text = "4"
        setPressListener(for: eightBtn, with: Command.keyClick(Key.n8))
        setPressListener(for: sixBtn, with: Command.keyClick(Key.n6))
        setPressListener(for: twoBtn, with: Command.keyClick(Key.n2))
        setPressListener(for: fourBtn, with: Command.keyClick(Key.n4))
        releaseShift()
    }
    
    // \[CommandCode\.keyClickFlags\.rawValue, (Key\.\w+)\.rawValue, Flags\.shift.rawValue\]
    // Command.keyClick($1, with: Key.shift)
    // \[CommandCode.keyClick.rawValue, (Key\.\w+)\.rawValue]
    // Command.keyClick($1)
    
    override func viewDidLoad() {
        super.viewDidLoad()
        
        pressShift = makeListener(with: Command.keyDown(Key.shift))
        releaseShift = makeListener(with: Command.keyUp(Key.shift))
        
        setPressListener(for: cutBtn, with: Command.keyClick(Key.x, with: Key.meta))
        setPressListener(for: copyBtn, with: Command.keyClick(Key.c, with: Key.meta))
        setPressListener(for: pasteBtn, with: Command.keyClick(Key.v, with: Key.meta))
        setPressListener(for: equalBtn, with: Command.keyClick(Key.equal))
        setPressListener(for: divideBtn, with: Command.keyClick(Key.slash))
        setPressListener(for: multiplyBtn, with: Command.keyClick(Key.n8, with: Key.shift))
        setPressListener(for: deleteBtn, with: Command.keyClick(Key.delete))
        
        setPressListener(for: lparenBtn, with: Command.keyClick(Key.n9, with: Key.shift))
        setPressListener(for: upBtn, with: Command.keyClick(Key.upArrow))
        setPressListener(for: rparenBtn, with: Command.keyClick(Key.n0, with: Key.shift))
        setPressListener(for: sevenBtn, with: Command.keyClick(Key.n7))
        setPressListener(for: eightBtn, with: Command.keyClick(Key.n8))
        setPressListener(for: nineBtn, with: Command.keyClick(Key.n9))
        setPressListener(for: subtractBtn, with: Command.keyClick(Key.minus))
        
        setPressListener(for: leftBtn, with: Command.keyClick(Key.leftArrow))
        setPressListener(for: dollarBtn, with: Command.keyClick(Key.n4, with: Key.shift))
        setPressListener(for: rightBtn, with: Command.keyClick(Key.rightArrow))
        setPressListener(for: fourBtn, with: Command.keyClick(Key.n4))
        setPressListener(for: fiveBtn, with: Command.keyClick(Key.n5))
        setPressListener(for: sixBtn, with: Command.keyClick(Key.n6))
        setPressListener(for: addBtn, with: Command.keyClick(Key.equal, with: Key.shift))
        
        setPressListener(for: lessBtn, with: Command.keyClick(Key.comma, with: Key.shift))
        setPressListener(for: downBtn, with: Command.keyClick(Key.downArrow))
        setPressListener(for: greaterBtn, with: Command.keyClick(Key.period, with: Key.shift))
        setPressListener(for: oneBtn, with: Command.keyClick(Key.n1))
        setPressListener(for: twoBtn, with: Command.keyClick(Key.n2))
        setPressListener(for: threeBtn, with: Command.keyClick(Key.n3))
        setPressListener(for: returnBtn, with: Command.keyClick(Key.return))
        
        setPressListener(for: caretBtn, with: Command.keyClick(Key.n6, with: Key.shift))
        setPressListener(for: percentBtn, with: Command.keyClick(Key.n5, with: Key.shift))
        setPressListener(for: commaBtn, with: Command.keyClick(Key.comma))
        setPressListener(for: zeroBtn, with: Command.keyClick(Key.n0))
        setPressListener(for: periodBtn, with: Command.keyClick(Key.period))
        
        shiftBtn.pressed = shiftPressed
        shiftBtn.released = shiftReleased
    }
}
