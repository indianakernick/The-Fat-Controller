// This file was generated automatically

enum CommandCode: UInt8, CaseIterable {
    case delay
    case keyDown
    case keyUp
    case keyClick
    case mouseMoveRel
    case mouseMoveAbs
    case mouseScroll
    case mouseDown
    case mouseUp
    case mouseClick
    case asciiCharDown
    case asciiCharUp
    case asciiChar
    case asciiString
    case unicodeCharDown
    case unicodeCharUp
    case unicodeChar
    case unicodeString
}

extension CommandCode: CustomStringConvertible {
    var description: String {
        switch self {
            case .delay: return "Delay"
            case .keyDown: return "Key Down"
            case .keyUp: return "Key Up"
            case .keyClick: return "Key Click"
            case .mouseMoveRel: return "Mouse Move Relative"
            case .mouseMoveAbs: return "Mouse Move Absolute"
            case .mouseScroll: return "Mouse Scroll"
            case .mouseDown: return "Mouse Down"
            case .mouseUp: return "Mouse Up"
            case .mouseClick: return "Mouse Click"
            case .asciiCharDown: return "ASCII Character Down"
            case .asciiCharUp: return "ASCII Character Up"
            case .asciiChar: return "ASCII Character"
            case .asciiString: return "ASCII String"
            case .unicodeCharDown: return "Unicode Character Down"
            case .unicodeCharUp: return "Unicode Character Up"
            case .unicodeChar: return "Unicode Character"
            case .unicodeString: return "Unicode String"
        }
    }
}

extension CommandCode: Enum {}
