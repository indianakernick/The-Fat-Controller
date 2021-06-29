//
//  Command.swift
//  Remote
//
//  Created by Indiana Kernick on 9/2/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import Foundation

fileprivate func setInt(_ buf: inout Data, index: Int, value: Int16) {
    buf[index] = UInt8((value >> 8) & 0xFF)
    buf[index + 1] = UInt8(value & 0xFF)
}

fileprivate func setInt(_ buf: inout Data, index: Int, value: UInt16) {
    buf[index] = UInt8((value >> 8) & 0xFF)
    buf[index + 1] = UInt8(value & 0xFF)
}

fileprivate func setChar(_ buf: inout Data, index: Int, value: UInt32) {
    buf[index] = UInt8((value >> 24) & 0xFF)
    buf[index + 1] = UInt8((value >> 16) & 0xFF)
    buf[index + 2] = UInt8((value >> 8) & 0xFF)
    buf[index + 3] = UInt8(value & 0xFF)
}

enum CommandData {
    // --- Delay --- //
    
    static func delay(_ delay: UInt16) -> Data {
        var buf = Data([CommandCode.delay.rawValue, 0, 0])
        setInt(&buf, index: 1, value: delay)
        return buf
    }
    
    // --- Key --- //
    
    static func keyDown(_ key: Key) -> Data {
        Data([CommandCode.keyDown.rawValue, key.rawValue])
    }
    
    static func keyUp(_ key: Key) -> Data {
        Data([CommandCode.keyUp.rawValue, key.rawValue])
    }
    
    static func keyClick(_ key: Key) -> Data {
        Data([CommandCode.keyClick.rawValue, key.rawValue])
    }
    
    static func keyClick(_ key: Key, with modifier: Key) -> Data {
        Data([
            CommandCode.keyDown.rawValue, modifier.rawValue,
            CommandCode.keyClick.rawValue, key.rawValue,
            CommandCode.keyUp.rawValue, modifier.rawValue
        ])
    }
    
    // --- Mouse --- //
    
    static func mouseMoveRel() -> Data {
        Data([CommandCode.mouseMoveRel.rawValue, 0, 0, 0, 0])
    }
    
    static func mouseMoveAbs() -> Data {
        Data([CommandCode.mouseMoveAbs.rawValue, 0, 0, 0, 0])
    }
    
    static func mouseScroll() -> Data {
        Data([CommandCode.mouseScroll.rawValue, 0, 0, 0, 0])
    }
    
    static func setMouseParams(_ buf: inout Data, x: Int16, y: Int16) {
        setInt(&buf, index: 1, value: x);
        setInt(&buf, index: 3, value: y);
    }
    
    static func mouseDown(_ button: MouseButton) -> Data {
        Data([CommandCode.mouseDown.rawValue, button.rawValue])
    }
    
    static func mouseUp(_ button: MouseButton) -> Data {
        Data([CommandCode.mouseUp.rawValue, button.rawValue])
    }
    
    static func mouseClick(_ button: MouseButton) -> Data {
        Data([CommandCode.mouseClick.rawValue, button.rawValue])
    }
    
    // --- Unicode --- //
    
    static func unicodeCharDown() -> Data {
        Data([CommandCode.unicodeCharDown.rawValue, 0, 0, 0, 0])
    }
    
    static func unicodeCharUp() -> Data {
        Data([CommandCode.unicodeCharUp.rawValue, 0, 0, 0, 0])
    }
    
    static func unicodeChar() -> Data {
        Data([CommandCode.unicodeChar.rawValue, 0, 0, 0, 0])
    }
    
    static func setUnicodeCharParam(_ buf: inout Data, char: Unicode.Scalar) {
        setChar(&buf, index: 1, value: char.value)
    }
    
    static func unicodeString(_ string: String) -> Data {
        var buf = Data([CommandCode.unicodeString.rawValue, 0, 0])
        let utf8 = string.data(using: .utf8)!
        assert(utf8.count <= UInt16.max)
        setInt(&buf, index: 1, value: UInt16(utf8.count))
        buf.append(utf8)
        return buf
    }
}
