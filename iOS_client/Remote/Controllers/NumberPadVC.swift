//
//  NumberPadVC.swift
//  Remote
//
//  Created by Indiana Kernick on 31/1/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import UIKit

class NumberPadVC: BasicVC {
    private var pressShift = {}
    private var releaseShift = {}
    private var landscapeButtons = [LabelButtonInput]()
    
    private func shiftPressed() {
        eightBtn.text = "↑"
        sixBtn.text = "→"
        twoBtn.text = "↓"
        fourBtn.text = "←"
        setPressListener(for: eightBtn, with: CommandData.keyClick(Key.upArrow, with: Key.shift))
        setPressListener(for: sixBtn, with: CommandData.keyClick(Key.rightArrow, with: Key.shift))
        setPressListener(for: twoBtn, with: CommandData.keyClick(Key.downArrow, with: Key.shift))
        setPressListener(for: fourBtn, with: CommandData.keyClick(Key.leftArrow, with: Key.shift))
        pressShift()
    }
    
    private func shiftReleased() {
        eightBtn.text = "8"
        sixBtn.text = "6"
        twoBtn.text = "2"
        fourBtn.text = "4"
        setPressListener(for: eightBtn, with: CommandData.keyClick(Key.n8))
        setPressListener(for: sixBtn, with: CommandData.keyClick(Key.n6))
        setPressListener(for: twoBtn, with: CommandData.keyClick(Key.n2))
        setPressListener(for: fourBtn, with: CommandData.keyClick(Key.n4))
        releaseShift()
    }
    
    private func setLandscapeButtons(hidden: Bool) {
        for btn in landscapeButtons {
            btn.isHidden = hidden
        }
    }
    
    // --- Interface Builder --- //
    
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
    
    // --- UIViewController --- //
    
    override func viewDidLoad() {
        super.viewDidLoad()
        
        pressShift = makeListener(with: CommandData.keyDown(Key.shift))
        releaseShift = makeListener(with: CommandData.keyUp(Key.shift))
        landscapeButtons = [
            cutBtn, copyBtn, pasteBtn,
            lparenBtn, upBtn, rparenBtn,
            leftBtn, dollarBtn, rightBtn,
            lessBtn, downBtn, greaterBtn,
            shiftBtn, caretBtn, percentBtn
        ]
        
        setPressListener(for: cutBtn, with: CommandData.keyClick(Key.x, with: Key.controlOrMeta))
        setPressListener(for: copyBtn, with: CommandData.keyClick(Key.c, with: Key.controlOrMeta))
        setPressListener(for: pasteBtn, with: CommandData.keyClick(Key.v, with: Key.controlOrMeta))
        setPressListener(for: equalBtn, with: CommandData.keyClick(Key.equal))
        setPressListener(for: divideBtn, with: CommandData.keyClick(Key.slash))
        setPressListener(for: multiplyBtn, with: CommandData.keyClick(Key.n8, with: Key.shift))
        setPressListener(for: deleteBtn, with: CommandData.keyClick(Key.deleteOrBackspace))
        
        setPressListener(for: lparenBtn, with: CommandData.keyClick(Key.n9, with: Key.shift))
        setPressListener(for: upBtn, with: CommandData.keyClick(Key.upArrow))
        setPressListener(for: rparenBtn, with: CommandData.keyClick(Key.n0, with: Key.shift))
        setPressListener(for: sevenBtn, with: CommandData.keyClick(Key.n7))
        setPressListener(for: eightBtn, with: CommandData.keyClick(Key.n8))
        setPressListener(for: nineBtn, with: CommandData.keyClick(Key.n9))
        setPressListener(for: subtractBtn, with: CommandData.keyClick(Key.minus))
        
        setPressListener(for: leftBtn, with: CommandData.keyClick(Key.leftArrow))
        setPressListener(for: dollarBtn, with: CommandData.keyClick(Key.n4, with: Key.shift))
        setPressListener(for: rightBtn, with: CommandData.keyClick(Key.rightArrow))
        setPressListener(for: fourBtn, with: CommandData.keyClick(Key.n4))
        setPressListener(for: fiveBtn, with: CommandData.keyClick(Key.n5))
        setPressListener(for: sixBtn, with: CommandData.keyClick(Key.n6))
        setPressListener(for: addBtn, with: CommandData.keyClick(Key.equal, with: Key.shift))
        
        setPressListener(for: lessBtn, with: CommandData.keyClick(Key.comma, with: Key.shift))
        setPressListener(for: downBtn, with: CommandData.keyClick(Key.downArrow))
        setPressListener(for: greaterBtn, with: CommandData.keyClick(Key.period, with: Key.shift))
        setPressListener(for: oneBtn, with: CommandData.keyClick(Key.n1))
        setPressListener(for: twoBtn, with: CommandData.keyClick(Key.n2))
        setPressListener(for: threeBtn, with: CommandData.keyClick(Key.n3))
        setPressListener(for: returnBtn, with: CommandData.keyClick(Key.returnOrEnter))
        
        setPressListener(for: caretBtn, with: CommandData.keyClick(Key.n6, with: Key.shift))
        setPressListener(for: percentBtn, with: CommandData.keyClick(Key.n5, with: Key.shift))
        setPressListener(for: commaBtn, with: CommandData.keyClick(Key.comma))
        setPressListener(for: zeroBtn, with: CommandData.keyClick(Key.n0))
        setPressListener(for: periodBtn, with: CommandData.keyClick(Key.period))
        
        shiftBtn.pressed = shiftPressed
        shiftBtn.released = shiftReleased
    }
    
    override func viewWillAppear(_ animated: Bool) {
        super.viewWillAppear(animated)
        let size = view.frame.size
        setLandscapeButtons(hidden: size.width < size.height)
    }
    
    override func viewWillTransition(to size: CGSize, with coordinator: UIViewControllerTransitionCoordinator) {
        super.viewWillTransition(to: size, with: coordinator)
        
        if size.width < size.height {
            // Need to hide the landscape buttons after the transition has
            // completed
            coordinator.animate(alongsideTransition: nil) { _ in
                self.setLandscapeButtons(hidden: true)
            }
        } else {
            setLandscapeButtons(hidden: false)
        }
    }
    
    // --- BasicVC --- //
    
    override func socketStatusChanged(_ status: SocketStatus) {
        super.socketStatusChanged(status)
        let size = view.frame.size
        setLandscapeButtons(hidden: size.width < size.height)
    }
}
