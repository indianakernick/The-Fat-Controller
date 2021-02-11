// This file was generated automatically

enum CommandCode: UInt8, CaseIterable {
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
    mouseClick,
    delay
}

extension CommandCode: CustomStringConvertible {
    var description: String {
        switch self {
            case .keyDown: return "Key down"
            case .keyUp: return "Key up"
            case .keyClick: return "Key click"
            case .mouseMoveRel: return "Mouse move rel"
            case .mouseMoveAbs: return "Mouse move abs"
            case .mouseWarp: return "Mouse warp"
            case .mouseScroll: return "Mouse scroll"
            case .mouseDown: return "Mouse down"
            case .mouseUp: return "Mouse up"
            case .mouseClick: return "Mouse click"
            case .delay: return "Delay"
        }
    }
}
