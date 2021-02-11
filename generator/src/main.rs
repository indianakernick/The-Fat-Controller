mod commands;
mod keys;
mod mouse_buttons;

use commands::*;
use keys::*;
use mouse_buttons::*;

use std::fs::File;
use std::io::Write;
use heck::{MixedCase, CamelCase};

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

fn generate_rust_enum(path: &str, name: &[u8], variants: &[&'static str]) -> std::io::Result<File> {
    let mut file = File::create(path)?;
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

fn generate_rust_from_string(file: &mut File, name: &[u8], variants: &[&'static str]) -> std::io::Result<()> {
    file.write_all(b"\nimpl std::str::FromStr for ")?;
    file.write_all(name)?;
    file.write_all(b" {\n    type Err = ();\n\n    fn from_str(s: &str) -> Result<Self, Self::Err> {\n        use ")?;
    file.write_all(name)?;
    file.write_all(b"::*;\n        match s {\n")?;

    for var in variants.iter() {
        file.write_all(b"            \"")?;
        file.write_all(var.to_camel_case().to_lowercase().as_bytes())?;
        file.write_all(b"\" => Ok(")?;
        file.write_all(to_rust_variant(var).as_bytes())?;
        file.write_all(b"),\n")?;
    }

    file.write_all(b"            _ => Err(()),\n        }\n    }\n}\n")?;

    Ok(())
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

fn generate_swift_enum(path: &str, name: &[u8], variants: &[&'static str]) -> std::io::Result<File> {
    let mut file = File::create(path)?;
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

fn generate_swift_to_string(file: &mut File, name: &[u8], variants: &[&'static str]) -> std::io::Result<()> {
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
        let mut file = generate_rust_enum("../src/command_code.rs", name, &COMMANDS).unwrap();
        generate_rust_from_string(&mut file, name, &COMMANDS).unwrap();
    }

    {
        let name = b"Key";
        let mut file = generate_rust_enum("../src/key.rs", name, &KEYS).unwrap();
        generate_rust_from_string(&mut file, name, &KEYS).unwrap();
    }

    {
        let name = b"MouseButton";
        let mut file = generate_rust_enum("../src/mouse_button.rs", name, &MOUSE_BUTTONS).unwrap();
        generate_rust_from_string(&mut file, name, &MOUSE_BUTTONS).unwrap();
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
