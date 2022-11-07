use std::process::Command;

fn main() {
    // Default to /dev/uinput on linux unless X11 is found
    if linux_session_type()
        .map(|x| x.contains("x11"))
        .unwrap_or(false)
    {
        println!("cargo:rustc-cfg=x11");
    }
}

fn linux_session_type() -> Result<String, Box<dyn std::error::Error>> {
    // Really rough implementation of this:
    // https://unix.stackexchange.com/a/325972/356153
    let output = Command::new("loginctl").output()?;
    let user = std::env::var("USER")?;
    let session = std::str::from_utf8(&output.stdout)?
        .lines()
        .find(|x| x.contains(&user))
        .unwrap_or("")
        .trim()
        .split(' ')
        .next()
        .unwrap_or("");
    let output = Command::new("loginctl")
        .arg("show-session")
        .arg(session)
        .arg("-p")
        .arg("Type")
        .output()?;
    Ok(std::str::from_utf8(&output.stdout)?.to_string())
}
