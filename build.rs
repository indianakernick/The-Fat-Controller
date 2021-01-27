use std::io::Write;
use heck::{ShoutySnakeCase, MixedCase};

type Variant = (&'static str, u8);

// /Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/System/Library/Frameworks/Carbon.framework/Versions/A/Frameworks/HIToolbox.framework/Versions/A/Headers/Events.h

const KEYS: [Variant; 113] = [
    // Dependent on keyboard layout
    ("A",                         0x00),
    ("S",                         0x01),
    ("D",                         0x02),
    ("F",                         0x03),
    ("H",                         0x04),
    ("G",                         0x05),
    ("Z",                         0x06),
    ("X",                         0x07),
    ("C",                         0x08),
    ("V",                         0x09),
    ("B",                         0x0B),
    ("Q",                         0x0C),
    ("W",                         0x0D),
    ("E",                         0x0E),
    ("R",                         0x0F),
    ("Y",                         0x10),
    ("T",                         0x11),
    ("N1",                        0x12),
    ("N2",                        0x13),
    ("N3",                        0x14),
    ("N4",                        0x15),
    ("N6",                        0x16),
    ("N5",                        0x17),
    ("Equal",                     0x18),
    ("N9",                        0x19),
    ("N7",                        0x1A),
    ("Minus",                     0x1B),
    ("N8",                        0x1C),
    ("N0",                        0x1D),
    ("RightBracket",              0x1E),
    ("O",                         0x1F),
    ("U",                         0x20),
    ("LeftBracket",               0x21),
    ("I",                         0x22),
    ("P",                         0x23),
    ("L",                         0x25),
    ("J",                         0x26),
    ("Quote",                     0x27),
    ("K",                         0x28),
    ("Semicolon",                 0x29),
    ("Backslash",                 0x2A),
    ("Comma",                     0x2B),
    ("Slash",                     0x2C),
    ("N",                         0x2D),
    ("M",                         0x2E),
    ("Period",                    0x2F),
    ("Grave",                     0x32),
    ("KeypadDecimal",             0x41),
    ("KeypadMultiply",            0x43),
    ("KeypadPlus",                0x45),
    ("KeypadClear",               0x47),
    ("KeypadDivide",              0x4B),
    ("KeypadEnter",               0x4C),
    ("KeypadMinus",               0x4E),
    ("KeypadEquals",              0x51),
    ("Keypad0",                   0x52),
    ("Keypad1",                   0x53),
    ("Keypad2",                   0x54),
    ("Keypad3",                   0x55),
    ("Keypad4",                   0x56),
    ("Keypad5",                   0x57),
    ("Keypad6",                   0x58),
    ("Keypad7",                   0x59),
    ("Keypad8",                   0x5B),
    ("Keypad9",                   0x5C),

    // Independent of keyboard layout
    ("Return",                    0x24),
    ("Tab",                       0x30),
    ("Space",                     0x31),
    ("Delete",                    0x33),
    ("Escape",                    0x35),
    ("Command",                   0x37),
    ("Shift",                     0x38),
    ("CapsLock",                  0x39),
    ("Option",                    0x3A),
    ("Control",                   0x3B),
    ("RightCommand",              0x36),
    ("RightShift",                0x3C),
    ("RightOption",               0x3D),
    ("RightControl",              0x3E),
    ("Function",                  0x3F),
    ("F17",                       0x40),
    ("VolumeUp",                  0x48),
    ("VolumeDown",                0x49),
    ("Mute",                      0x4A),
    ("F18",                       0x4F),
    ("F19",                       0x50),
    ("F20",                       0x5A),
    ("F5",                        0x60),
    ("F6",                        0x61),
    ("F7",                        0x62),
    ("F3",                        0x63),
    ("F8",                        0x64),
    ("F9",                        0x65),
    ("F11",                       0x67),
    ("F13",                       0x69),
    ("F16",                       0x6A),
    ("F14",                       0x6B),
    ("F10",                       0x6D),
    ("F12",                       0x6F),
    ("F15",                       0x71),
    ("Help",                      0x72),
    ("Home",                      0x73),
    ("PageUp",                    0x74),
    ("ForwardDelete",             0x75),
    ("F4",                        0x76),
    ("End",                       0x77),
    ("F2",                        0x78),
    ("PageDown",                  0x79),
    ("F1",                        0x7A),
    ("LeftArrow",                 0x7B),
    ("RightArrow",                0x7C),
    ("DownArrow",                 0x7D),
    ("UpArrow",                   0x7E),
];

const FLAGS: [Variant; 6] = [
    ("Null",     0b00000),
    ("CapsLock", 0b00001),
    ("Shift",    0b00010),
    ("Control",  0b00100),
    ("Alt",      0b01000),
    ("Command",  0b10000),
];

const MOUSE_BUTTONS: [Variant; 3] = [
    ("Left",   0),
    ("Right",  1),
    ("Middle", 2),
];

const COMMANDS: [Variant; 12] = [
    ("MouseMoveTo",       0),
    ("MouseMoveRelative", 1),
    ("MouseDown",         2),
    ("MouseUp",           3),
    ("MouseClick",        4),
    ("MouseNthClick",     5),
    ("MouseScrollX",      6),
    ("MouseScrollY",      7),
    ("KeyDown",           8),
    ("KeyUp",             9),
    ("KeyClick",          10),
    ("KeyClickFlags",     11),
];

const COMMENT: &[u8; 40] = b"// This file was generated by build.rs\n\n";

fn generate_js_enum(path: &str, variants: &[Variant]) -> std::io::Result<()> {
    let mut file = std::fs::File::create(path)?;
    file.write_all(COMMENT)?;

    for var in variants.iter() {
        file.write_all(b"export const ")?;
        file.write_all(var.0.to_shouty_snake_case().as_bytes())?;
        file.write_all(b" = ")?;
        file.write_all(var.1.to_string().as_bytes())?;
        file.write_all(b";\n")?;
    }

    Ok(())
}

fn generate_rust_from_str(
    file: &mut std::fs::File,
    name: &[u8],
    variants: &[Variant],
    to_string_name: impl Fn(&[u8]) -> &[u8],
) -> std::io::Result<()> {
    file.write_all(b"\nimpl std::str::FromStr for ")?;
    file.write_all(name)?;
    file.write_all(b" {\n    type Err = ();\n\n    fn from_str(s: &str) -> Result<Self, Self::Err> {\n        use ")?;
    file.write_all(name)?;
    file.write_all(b"::*;\n        match s {\n")?;

    for var in variants.iter() {
        file.write_all(b"            \"")?;
        file.write_all(to_string_name(var.0.to_mixed_case().to_ascii_lowercase().as_bytes()))?;
        file.write_all(b"\" => Ok(")?;
        file.write_all(var.0.as_bytes())?;
        file.write_all(b"),\n")?
    }

    file.write_all(b"            _ => Err(()),\n        }\n    }\n}\n")?;

    Ok(())
}

fn generate_rust_from_byte(
    file: &mut std::fs::File,
    name: &[u8],
    variants: &[Variant],
) -> std::io::Result<()> {
    file.write_all(b"\nimpl std::convert::TryFrom<u8> for ")?;
    file.write_all(name)?;
    file.write_all(b" {\n    type Error = ();\n\n    fn try_from(b: u8) -> Result<Self, Self::Error> {\n        use ")?;
    file.write_all(name)?;
    file.write_all(b"::*;\n        match b {\n")?;

    for var in variants.iter() {
        file.write_all(b"            ")?;
        file.write_all(var.1.to_string().as_bytes())?;
        file.write_all(b" => Ok(")?;
        file.write_all(var.0.as_bytes())?;
        file.write_all(b"),\n")?;
    }

    file.write_all(b"            _ => Err(()),\n        }\n    }\n}\n")?;

    Ok(())
}

fn generate_rust_enum(path: &str, name: &[u8], variants: &[Variant]) -> std::io::Result<std::fs::File> {
    let mut file = std::fs::File::create(path)?;
    file.write_all(COMMENT)?;

    file.write_all(b"#[repr(u8)]\n#[derive(Clone, Copy, Debug)]\npub enum ")?;
    file.write_all(name)?;
    file.write_all(b" {\n")?;

    for var in variants.iter() {
        file.write_all(b"    ")?;
        file.write_all(var.0.as_bytes())?;
        file.write_all(b" = ")?;
        file.write_all(var.1.to_string().as_bytes())?;
        file.write_all(b",\n")?;
    }

    file.write_all(b"}\n")?;

    Ok(file)
}

fn generate_rust_bitflags(path: &str, name: &[u8], variants: &[Variant]) -> std::io::Result<()> {
    let mut file = std::fs::File::create(path)?;
    file.write_all(COMMENT)?;

    file.write_all(b"use bitflags::bitflags;\n\nbitflags! {\n    pub struct ")?;
    file.write_all(name)?;
    file.write_all(b": u8 {\n")?;

    for var in variants.iter() {
        file.write_all(b"        const ")?;
        file.write_all(var.0.to_shouty_snake_case().as_bytes())?;
        file.write_all(b" = ")?;
        file.write_all(var.1.to_string().as_bytes())?;
        file.write_all(b";\n")?;
    }

    file.write_all(b"    }\n}\n")?;

    Ok(())
}

fn main() {
    generate_js_enum("client/src/pages/common/Key.js", &KEYS).unwrap();
    generate_js_enum("client/src/pages/common/Flags.js", &FLAGS).unwrap();
    generate_js_enum("client/src/pages/common/MouseButton.js", &MOUSE_BUTTONS).unwrap();
    generate_js_enum("client/src/pages/common/CommandCode.js", &COMMANDS).unwrap();

    {
        let name = b"Key";
        let mut file = generate_rust_enum("src/macos/key_enum.rs", name, &KEYS).unwrap();
        generate_rust_from_byte(&mut file, name, &KEYS).unwrap();
        generate_rust_from_str(&mut file, name, &KEYS, |var_name| {
            if var_name.len() == 2 && var_name[0] == b'n' && var_name[1].is_ascii_digit() {
                &var_name[1..]
            } else {
                var_name
            }
        }).unwrap();
    }

    generate_rust_bitflags("src/macos/flags_enum.rs", b"Flags", &FLAGS).unwrap();

    {
        let name = b"MouseButton";
        let mut file = generate_rust_enum("src/macos/mouse_button_enum.rs", name, &MOUSE_BUTTONS).unwrap();
        generate_rust_from_byte(&mut file, name, &MOUSE_BUTTONS).unwrap();
        generate_rust_from_str(&mut file, name, &MOUSE_BUTTONS, |var_name| {
            match var_name[0] {
                b'l' => b"mouseleft",
                b'r' => b"mouseright",
                b'm' => b"mousemiddle",
                _ => panic!(),
            }
        }).unwrap();
    }

    {
        let name = b"CommandCode";
        let mut file = generate_rust_enum("src/macos/command_code_enum.rs", name, &COMMANDS).unwrap();
        generate_rust_from_byte(&mut file, name, &COMMANDS).unwrap();
    }

    println!("cargo:rerun-if-changed=client/public/click.html");
    println!("cargo:rerun-if-changed=client/public/number.html");
    println!("cargo:rerun-if-changed=client/public/press.html");
    println!("cargo:rerun-if-changed=client/public/slide.html");
    println!("cargo:rerun-if-changed=client/public/trackpad.html");

    println!("cargo:rerun-if-changed=client/src/pages/click/index.js");
    println!("cargo:rerun-if-changed=client/src/pages/click/styles.scss");
    println!("cargo:rerun-if-changed=client/src/pages/common/bootstrap.scss");
    println!("cargo:rerun-if-changed=client/src/pages/common/createButton.js");
    println!("cargo:rerun-if-changed=client/src/pages/common/SocketManager.js");
    println!("cargo:rerun-if-changed=client/src/pages/common/styles.scss");
    println!("cargo:rerun-if-changed=client/src/pages/number/index.js");
    println!("cargo:rerun-if-changed=client/src/pages/number/styles.scss");
    println!("cargo:rerun-if-changed=client/src/pages/press/index.js");
    println!("cargo:rerun-if-changed=client/src/pages/press/styles.scss");
    println!("cargo:rerun-if-changed=client/src/pages/slide/index.js");
    println!("cargo:rerun-if-changed=client/src/pages/slide/styles.scss");
    println!("cargo:rerun-if-changed=client/src/pages/trackpad/index.js");
    println!("cargo:rerun-if-changed=client/src/pages/trackpad/styles.scss");

    println!("cargo:rerun-if-changed=client/webpack.config.js");

    let build = match std::env::var("PROFILE").unwrap().as_str() {
        "debug" => "build-dev",
        "release" => "build-prod",
        _ => panic!()
    };

    let status = std::process::Command::new("npm")
        .arg("run")
        .arg(build)
        .current_dir("client")
        .status()
        .unwrap();
    assert!(status.success());
}
