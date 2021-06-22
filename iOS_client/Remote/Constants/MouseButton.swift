// This file was generated automatically

enum MouseButton: UInt8, CaseIterable {
    case left
    case right
    case middle
}

extension MouseButton: CustomStringConvertible {
    var description: String {
        switch self {
            case .left: return "Left"
            case .right: return "Right"
            case .middle: return "Middle"
        }
    }
}
