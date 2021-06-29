//
//  CommandStruct.swift
//  Remote
//
//  Created by Indiana Kernick on 29/6/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import Foundation

// Using an enum with associated values just ended up being really messy. You
// have to switch on the enum and then reassign if you want to modify the
// associated values. This is especially bad for x,y where there are two of them
// and you need to change one at a time.
struct CommandStruct {
    var code: CommandCode
    var key: Key
    var button: MouseButton
    var delay: UInt16
    var x: Int16
    var y: Int16
    var char: Unicode.Scalar
    var string: String
    
    init() {
        code = .delay
        key = .space
        button = .left
        delay = 0
        x = 0
        y = 0
        char = "a"
        string = ""
    }
    
    mutating func normalize() {
        let key = self.key
        let button = self.button
        let delay = self.delay
        let x = self.x
        let y = self.y
        let char = self.char
        let string = self.string
        
        self.key = .space
        self.button = .left
        self.delay = 0
        self.x = 0
        self.y = 0
        self.char = "a"
        self.string = ""
        
        switch code {
        case .delay:
            self.delay = delay
        case .keyDown:
            self.key = key
        case .keyUp:
            self.key = key
        case .keyClick:
            self.key = key
        case .mouseMoveRel:
            self.x = x
            self.y = y
        case .mouseMoveAbs:
            self.x = x
            self.y = y
        case .mouseScroll:
            self.x = x
            self.y = y
        case .mouseDown:
            self.button = button
        case .mouseUp:
            self.button = button
        case .mouseClick:
            self.button = button
        case .unicodeCharDown:
            self.char = char
        case .unicodeCharUp:
            self.char = char
        case .unicodeChar:
            self.char = char
        case .unicodeString:
            self.string = string
        default:
            assert(false)
        }
    }
    
    var parameterDescription: String {
        get {
            switch code {
            case .delay:
                return String(delay)
            case .keyDown:
                return key.description
            case .keyUp:
                return key.description
            case .keyClick:
                return key.description
            case .mouseMoveRel:
                return "\(x), \(y)"
            case .mouseMoveAbs:
                return "\(x), \(y)"
            case .mouseScroll:
                return "\(x), \(y)"
            case .mouseDown:
                return button.description
            case .mouseUp:
                return button.description
            case .mouseClick:
                return button.description
            case .unicodeCharDown:
                return String(char)
            case .unicodeCharUp:
                return String(char)
            case .unicodeChar:
                return String(char)
            case .unicodeString:
                return string
            default:
                assert(false)
            }
        }
    }
    
    var data: Data {
        get {
            switch code {
            case .delay:
                return CommandData.delay(delay)
            case .keyDown:
                return CommandData.keyDown(key)
            case .keyUp:
                return CommandData.keyUp(key)
            case .keyClick:
                return CommandData.keyClick(key)
            case .mouseMoveRel:
                var buf = CommandData.mouseMoveRel()
                CommandData.setMouseParams(&buf, x: x, y: y)
                return buf
            case .mouseMoveAbs:
                var buf = CommandData.mouseMoveAbs()
                CommandData.setMouseParams(&buf, x: x, y: y)
                return buf
            case .mouseScroll:
                var buf = CommandData.mouseScroll()
                CommandData.setMouseParams(&buf, x: x, y: y)
                return buf
            case .mouseDown:
                return CommandData.mouseDown(button)
            case .mouseUp:
                return CommandData.mouseUp(button)
            case .mouseClick:
                return CommandData.mouseClick(button)
            case .unicodeCharDown:
                var buf = CommandData.unicodeCharDown()
                CommandData.setUnicodeCharParam(&buf, char: char)
                return buf
            case .unicodeCharUp:
                var buf = CommandData.unicodeCharUp()
                CommandData.setUnicodeCharParam(&buf, char: char)
                return buf
            case .unicodeChar:
                var buf = CommandData.unicodeChar()
                CommandData.setUnicodeCharParam(&buf, char: char)
                return buf
            case .unicodeString:
                return CommandData.unicodeString(string)
            default:
                assert(false)
            }
        }
    }
}
