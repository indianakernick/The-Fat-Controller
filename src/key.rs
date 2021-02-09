#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum Key {
    // Modifier keys
    CapsLock,
    Shift,
    Control,
    Alt,
    Meta, // Command on macOS. Windows key on Windows.
    ControlOrMeta, // Command on macOS. Control on Windows.
    RightShift,
    RightControl,
    RightAlt,
    RightMeta, // Command on macOS. Windows key on Windows.
    RightControlOrMeta, // Command on macOS. Control on Windows.
    Fn,

    // Controls and symbols
    Return,
    Escape,
    Delete,
    ForwardDelete,
    Tab,
    Space,
    Minus,
    Equal,
    LeftBracket,
    RightBracket,
    Backslash,
    Semicolon,
    Quote,
    Grave,
    Comma,
    Period,
    Slash,

    // Arrow keys
    UpArrow,
    RightArrow,
    DownArrow,
    LeftArrow,
    PageUp,
    PageDown,
    Home,
    End,

    // Letter keys
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    // Number keys
    N0,
    N1,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,

    // Keypad number keys
    Keypad0,
    Keypad1,
    Keypad2,
    Keypad3,
    Keypad4,
    Keypad5,
    Keypad6,
    Keypad7,
    Keypad8,
    Keypad9,

    // Keypad keys
    KeypadClear,
    KeypadEquals,
    KeypadDivide,
    KeypadMultiply,
    KeypadMinus,
    KeypadPlus,
    KeypadEnter,
    KeypadDecimal,

    // Function keys
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,

    // Media controls
    FastForward,
    Rewind,
    PlayPause,
    VolumeUp,
    VolumeDown,
    Mute,
}

impl Key {
    pub const COUNT: u8 = Self::Mute as u8 + 1;
}
