//
//  CommandCode.swift
//  Remote
//
//  Created by Indiana Kernick on 28/1/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

enum CommandCode: UInt8 {
    case
    keyDown,
    keyUp,
    keyClick,
    mouseMoveRel,
    mouseMoveAbs,
    mouseWarp,
    mouseScroll,
    mouseDown,
    mouseUp,
    mouseClick
}
