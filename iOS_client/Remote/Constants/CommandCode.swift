// This file was generated automatically

enum CommandCode: UInt8, CaseIterable {
    case
    delay,
    keyDown,
    keyUp,
    keyClick,
    mouseMoveRel,
    mouseMoveAbs,
    mouseScroll,
    mouseDown,
    mouseUp,
    mouseClick,
    asciiCharDown,
    asciiCharUp,
    asciiChar,
    asciiString,
    unicodeCharDown,
    unicodeCharUp,
    unicodeChar,
    unicodeString
}

extension CommandCode: CustomStringConvertible {
    var description: String {
        switch self {
            case .delay: return "Delay"
            case .keyDown: return "Key down"
            case .keyUp: return "Key up"
            case .keyClick: return "Key click"
            case .mouseMoveRel: return "Mouse move rel"
            case .mouseMoveAbs: return "Mouse move abs"
            case .mouseScroll: return "Mouse scroll"
            case .mouseDown: return "Mouse down"
            case .mouseUp: return "Mouse up"
            case .mouseClick: return "Mouse click"
            case .asciiCharDown: return "ASCII char down"
            case .asciiCharUp: return "ASCII char up"
            case .asciiChar: return "ASCII char"
            case .asciiString: return "ASCII string"
            case .unicodeCharDown: return "Unicode char down"
            case .unicodeCharUp: return "Unicode char up"
            case .unicodeChar: return "Unicode char"
            case .unicodeString: return "Unicode string"
        }
    }
}
