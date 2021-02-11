use std::io::Write;
use heck::{MixedCase, CamelCase};

const COMMANDS: [&'static str; 11] = [
    "Key down",
    "Key up",
    "Key click",
    "Mouse move rel",
    "Mouse move abs",
    "Mouse warp",
    "Mouse scroll",
    "Mouse down",
    "Mouse up",
    "Mouse click",
    "Delay",
];

const KEYS: [&'static str; 109] = [
    // Modifier keys
    "Caps Lock",
    "Shift",
    "Control",
    "Alt",
    "Meta", // Command on macOS. Windows key on Windows.
    "Control or Meta", // Command on macOS. Control on Windows.
    "Right Shift",
    "Right Control",
    "Right Alt",
    "Right Meta", // Command on macOS. Windows key on Windows.
    "Right Control or Meta", // Command on macOS. Control on Windows.
    "Fn",

    // Controls and symbols
    "Return",
    "Escape",
    "Delete",
    "Forward Delete",
    "Tab",
    "Space",
    "Minus",
    "Equal",
    "Left Bracket",
    "Right Bracket",
    "Backslash",
    "Semicolon",
    "Quote",
    "Grave",
    "Comma",
    "Period",
    "Slash",

    // Arrow keys
    "Up Arrow",
    "Right Arrow",
    "Down Arrow",
    "Left Arrow",
    "Page Up",
    "Page Down",
    "Home",
    "End",

    // Letter keys
    "A",
    "B",
    "C",
    "D",
    "E",
    "F",
    "G",
    "H",
    "I",
    "J",
    "K",
    "L",
    "M",
    "N",
    "O",
    "P",
    "Q",
    "R",
    "S",
    "T",
    "U",
    "V",
    "W",
    "X",
    "Y",
    "Z",

    // Number keys
    "0",
    "1",
    "2",
    "3",
    "4",
    "5",
    "6",
    "7",
    "8",
    "9",

    // Number pad numbers
    "Numpad 0",
    "Numpad 1",
    "Numpad 2",
    "Numpad 3",
    "Numpad 4",
    "Numpad 5",
    "Numpad 6",
    "Numpad 7",
    "Numpad 8",
    "Numpad 9",

    // Number pad keys
    "Numpad Clear",
    "Numpad Equals",
    "Numpad Divide",
    "Numpad Multiply",
    "Numpad Minus",
    "Numpad Plus",
    "Numpad Enter",
    "Numpad Decimal",

    // Function keys
    "F1",
    "F2",
    "F3",
    "F4",
    "F5",
    "F6",
    "F7",
    "F8",
    "F9",
    "F10",
    "F11",
    "F12",

    // Media controls
    "Fast-Forward",
    "Rewind",
    "Play/Pause",
    "Volume Up",
    "Volume Down",
    "Mute",
];

const MOUSE_BUTTONS: [&'static str; 3] = [
    "Left",
    "Right",
    "Middle",
];

const COMMENT: &[u8; 42] = b"// This file was generated automatically\n\n";

fn to_rust_variant(var: &str) -> String {
    if var.chars().next().unwrap().is_ascii_digit() {
        let mut string = String::from("N");
        string += var;
        string.to_camel_case()
    } else {
        var.to_camel_case()
    }
}

fn generate_rust_enum(path: &str, name: &[u8], variants: &[&'static str]) -> std::io::Result<std::fs::File> {
    let mut file = std::fs::File::create(path)?;
    file.write_all(COMMENT)?;

    file.write_all(b"#[repr(u8)]\n#[derive(Copy, Clone, Debug, Eq, PartialEq)]\npub enum ")?;
    file.write_all(name)?;
    file.write_all(b" {\n")?;

    for var in variants.iter() {
        file.write_all(b"    ")?;
        file.write_all(to_rust_variant(var).as_bytes())?;
        file.write_all(b",\n")?;
    }

    file.write_all(b"}\n\nimpl ")?;
    file.write_all(name)?;
    file.write_all(b" {\n    pub const COUNT: u8 = ")?;
    file.write_all(variants.len().to_string().as_bytes())?;
    file.write_all(b";\n}\n")?;

    Ok(file)
}

fn to_swift_variant(var: &str) -> String {
    if var.chars().next().unwrap().is_ascii_digit() {
        let mut string = String::from("n");
        string += var;
        string.to_mixed_case()
    } else if var == "Return" {
        String::from("`return`")
    } else {
        var.to_mixed_case()
    }
}

fn generate_swift_enum(path: &str, name: &[u8], variants: &[&'static str]) -> std::io::Result<std::fs::File> {
    let mut file = std::fs::File::create(path)?;
    file.write_all(COMMENT)?;

    file.write_all(b"enum ")?;
    file.write_all(name)?;
    file.write_all(b": UInt8, CaseIterable {\n    case\n")?;

    for var in variants.iter().take(variants.len() - 1) {
        file.write_all(b"    ")?;
        file.write_all(to_swift_variant(var).as_bytes())?;
        file.write_all(b",\n")?;
    }

    file.write_all(b"    ")?;
    file.write_all(to_swift_variant(variants[variants.len() - 1]).as_bytes())?;
    file.write_all(b"\n}\n")?;

    Ok(file)
}

fn generate_swift_to_string(file: &mut std::fs::File, name: &[u8], variants: &[&'static str]) -> std::io::Result<()> {
    file.write_all(b"\nextension ")?;
    file.write_all(name)?;
    file.write_all(b": CustomStringConvertible {\n    var description: String {\n        switch self {\n")?;

    for var in variants.iter() {
        file.write_all(b"            case .")?;
        file.write_all(to_swift_variant(var).as_bytes())?;
        file.write_all(b": return \"")?;
        file.write_all(var.as_bytes())?;
        file.write_all(b"\"\n")?;
    }

    file.write_all(b"        }\n    }\n}\n")?;

    Ok(())
}

fn main() {
    {
        let name = b"CommandCode";
        let mut _file = generate_rust_enum("../src/command_code.rs", name, &COMMANDS).unwrap();
    }

    {
        let name = b"Key";
        let mut _file = generate_rust_enum("../src/key.rs", name, &KEYS).unwrap();
    }

    {
        let name = b"MouseButton";
        let mut _file = generate_rust_enum("../src/mouse_button.rs", name, &MOUSE_BUTTONS).unwrap();
    }

    {
        let name = b"CommandCode";
        let mut file = generate_swift_enum("../iOS_client/Remote/Constants/CommandCode.swift", name, &COMMANDS).unwrap();
        generate_swift_to_string(&mut file, name, &COMMANDS).unwrap();
    }

    {
        let name = b"Key";
        let mut file = generate_swift_enum("../iOS_client/Remote/Constants/Key.swift", name, &KEYS).unwrap();
        generate_swift_to_string(&mut file, name, &KEYS).unwrap();
    }

    {
        let name = b"MouseButton";
        let mut file = generate_swift_enum("../iOS_client/Remote/Constants/MouseButton.swift", name, &MOUSE_BUTTONS).unwrap();
        generate_swift_to_string(&mut file, name, &MOUSE_BUTTONS).unwrap();
    }
}
