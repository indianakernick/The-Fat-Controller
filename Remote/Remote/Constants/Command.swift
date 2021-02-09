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

enum Command {
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
    
    static func mouseMoveRel() -> Data {
        Data([CommandCode.mouseMoveRel.rawValue, 0, 0, 0, 0])
    }
    
    static func mouseMoveAbs() -> Data {
        Data([CommandCode.mouseMoveAbs.rawValue, 0, 0, 0, 0])
    }
    
    static func mouseWarp() -> Data {
        Data([CommandCode.mouseWarp.rawValue, 0, 0, 0, 0])
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
}
