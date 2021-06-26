use std::{fs::File, io::Write};

fn swift_identifier_name<E: tfc::Enum>(var: E) -> String {
    let mut chars = var.identifier_name().chars();
    String::from(chars.next().unwrap().to_ascii_lowercase()) + chars.as_str()
}

fn generate_swift_enum<E: tfc::Enum>() -> std::io::Result<()> {
    let mut file = File::create(format!("../iOS_client/Remote/Constants/{}.swift", E::NAME))?;

    file.write_all(b"// This file was generated automatically\n\nenum ")?;
    file.write_all(E::NAME.as_bytes())?;
    file.write_all(b": UInt8, CaseIterable {\n")?;

    for var in E::iter() {
        file.write_all(b"    case ")?;
        file.write_all(swift_identifier_name(var).as_bytes())?;
        file.write_all(b"\n")?;
    }

    file.write_all(b"}\n\nextension ")?;
    file.write_all(E::NAME.as_bytes())?;
    file.write_all(b": CustomStringConvertible {\n    var description: String {\n        switch self {\n")?;

    for var in E::iter() {
        file.write_all(b"            case .")?;
        file.write_all(swift_identifier_name(var).as_bytes())?;
        file.write_all(b": return \"")?;
        file.write_all(var.display_name().as_bytes())?;
        file.write_all(b"\"\n")?;
    }

    file.write_all(b"        }\n    }\n}\n\nextension ")?;
    file.write_all(E::NAME.as_bytes())?;
    file.write_all(b": Enum {}\n")?;

    Ok(())
}

fn main() {
    generate_swift_enum::<tfc::CommandCode>().unwrap();
    generate_swift_enum::<tfc::Key>().unwrap();
    generate_swift_enum::<tfc::MouseButton>().unwrap();
}
