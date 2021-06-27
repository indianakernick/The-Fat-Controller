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
    
    private func getReplacedText(range: NSRange, string: String) -> String {
        var newString = text ?? ""
        let stringRange = Range<String.Index>(range, in: newString)!
        newString.replaceSubrange(stringRange, with: string)
        return newString
    }
    
    // --- TextInput --- //
    
    var textChanged: (String) -> Void = { text in }
    
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
        case .char, .string:
            keyboardType = .default
        }
    }
    
    // --- UITextFieldDelegate --- //
    
    func textField(_ textField: UITextField, shouldChangeCharactersIn range: NSRange, replacementString string: String) -> Bool {
        let isDigit: (Character) -> Bool = { char in
            char.isASCII && char.isWholeNumber
        }
        
        switch mode {
        case .int, .uint:
            if !string.allSatisfy(isDigit) {
                return false
            }
            let newString = getReplacedText(range: range, string: string)
            if newString.count == 0 {
                textChanged("0")
                return true
            }
            guard let value = Int(newString) else {
                return false
            }
            if String(value) != newString {
                return false
            }
            if mode == .int && (value < Int16.min || value > Int16.max) {
                return false
            }
            if mode == .uint && value > UInt16.max {
                return false
            }
            textChanged(newString)
            return true
            
        case .char:
            // Whenever a single unicode scalar is typed, that single scalar
            // will replace the contents of the text field. This might seem a
            // little odd to the user but this ensures that there is always
            // exactly one unicode scalar in the text field.
            if string.count != 1 || string.unicodeScalars.count > 1 {
                return false
            }
            text = string
            textChanged(string)
            return false
            
        case .string:
            textChanged(getReplacedText(range: range, string: string))
            return true
        }
    }
    
    func textFieldDidEndEditing(_ textField: UITextField) {
        if (mode == .int || mode == .uint) && text?.isEmpty ?? true {
            text = "0"
        }
    }
}
