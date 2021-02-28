//
//  NumberPadViewController.swift
//  Remote
//
//  Created by Indiana Kernick on 31/1/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

class NumberPadViewController: BasicViewController {
    @IBOutlet weak var cutBtn: LabelButtonInput!
    @IBOutlet weak var copyBtn: LabelButtonInput!
    @IBOutlet weak var pasteBtn: LabelButtonInput!
    @IBOutlet weak var equalBtn: LabelButtonInput!
    @IBOutlet weak var divideBtn: LabelButtonInput!
    @IBOutlet weak var multiplyBtn: LabelButtonInput!
    @IBOutlet weak var deleteBtn: LabelButtonInput!
    @IBOutlet weak var lparenBtn: LabelButtonInput!
    @IBOutlet weak var upBtn: LabelButtonInput!
    @IBOutlet weak var rparenBtn: LabelButtonInput!
    @IBOutlet weak var sevenBtn: LabelButtonInput!
    @IBOutlet weak var eightBtn: LabelButtonInput!
    @IBOutlet weak var nineBtn: LabelButtonInput!
    @IBOutlet weak var subtractBtn: LabelButtonInput!
    @IBOutlet weak var leftBtn: LabelButtonInput!
    @IBOutlet weak var dollarBtn: LabelButtonInput!
    @IBOutlet weak var rightBtn: LabelButtonInput!
    @IBOutlet weak var fourBtn: LabelButtonInput!
    @IBOutlet weak var fiveBtn: LabelButtonInput!
    @IBOutlet weak var sixBtn: LabelButtonInput!
    @IBOutlet weak var addBtn: LabelButtonInput!
    @IBOutlet weak var lessBtn: LabelButtonInput!
    @IBOutlet weak var downBtn: LabelButtonInput!
    @IBOutlet weak var greaterBtn: LabelButtonInput!
    @IBOutlet weak var oneBtn: LabelButtonInput!
    @IBOutlet weak var twoBtn: LabelButtonInput!
    @IBOutlet weak var threeBtn: LabelButtonInput!
    @IBOutlet weak var returnBtn: LabelButtonInput!
    @IBOutlet weak var shiftBtn: LabelButtonInput!
    @IBOutlet weak var caretBtn: LabelButtonInput!
    @IBOutlet weak var percentBtn: LabelButtonInput!
    @IBOutlet weak var commaBtn: LabelButtonInput!
    @IBOutlet weak var zeroBtn: LabelButtonInput!
    @IBOutlet weak var periodBtn: LabelButtonInput!
    
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
    
    override func viewDidLoad() {
        super.viewDidLoad()
        
        pressShift = makeListener(with: Command.keyDown(Key.shift))
        releaseShift = makeListener(with: Command.keyUp(Key.shift))
        
        setPressListener(for: cutBtn, with: Command.keyClick(Key.x, with: Key.controlOrMeta))
        setPressListener(for: copyBtn, with: Command.keyClick(Key.c, with: Key.controlOrMeta))
        setPressListener(for: pasteBtn, with: Command.keyClick(Key.v, with: Key.controlOrMeta))
        setPressListener(for: equalBtn, with: Command.keyClick(Key.equal))
        setPressListener(for: divideBtn, with: Command.keyClick(Key.slash))
        setPressListener(for: multiplyBtn, with: Command.keyClick(Key.n8, with: Key.shift))
        setPressListener(for: deleteBtn, with: Command.keyClick(Key.deleteOrBackspace))
        
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
        setPressListener(for: returnBtn, with: Command.keyClick(Key.returnOrEnter))
        
        setPressListener(for: caretBtn, with: Command.keyClick(Key.n6, with: Key.shift))
        setPressListener(for: percentBtn, with: Command.keyClick(Key.n5, with: Key.shift))
        setPressListener(for: commaBtn, with: Command.keyClick(Key.comma))
        setPressListener(for: zeroBtn, with: Command.keyClick(Key.n0))
        setPressListener(for: periodBtn, with: Command.keyClick(Key.period))
        
        shiftBtn.pressed = shiftPressed
        shiftBtn.released = shiftReleased
    }
}
