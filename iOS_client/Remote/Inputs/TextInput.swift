//
//  TextInput.swift
//  Remote
//
//  Created by Indiana Kernick on 26/6/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import UIKit

class TextInput: UITextField, UITextFieldDelegate {
    private var mode = Mode.string
    
    // --- TextInput --- //
    
    enum Mode {
        case int, uint, char, string
    }
    
    func setIndent(_ indent: CGFloat) {
        let left = UIView(frame: CGRect(
            x: 0, y: 0,
            width: indent, height: frame.size.height
        ))
        left.backgroundColor = backgroundColor
        leftView = left
        leftViewMode = .always
    }
    
    func setMode(_ mode: Mode) {
        self.mode = mode
        delegate = self
        
        switch mode {
        case .int:
            keyboardType = .decimalPad // going to use . as +/- button
        case .uint:
            keyboardType = .numberPad
            break
        case .char:
            fallthrough
        case .string:
            keyboardType = .default
        }
    }
    
    // --- UITextFieldDelegate --- //
    
    func textField(_ textField: UITextField, shouldChangeCharactersIn range: NSRange, replacementString string: String) -> Bool {
        // To handle int and uint properly, it will probably be easier to apply
        // the change to the string and then check if the resulting string is
        // valid. Trying to be "smart" about it will just result in messy code.
        
        let isDigit: (Character) -> Bool = { char in
            char.isASCII && char.isWholeNumber
        }
        
        switch mode {
        case .int:
            return string.allSatisfy(isDigit)
        case .uint:
            return string.allSatisfy(isDigit)
        case .char:
            if string.count > 1 || string.unicodeScalars.count > 1 {
                return false
            }
            if string.count == 0 {
                return true
            }
            return range.length == (text?.count ?? 0)
        case .string:
            return true
        }
    }
}
