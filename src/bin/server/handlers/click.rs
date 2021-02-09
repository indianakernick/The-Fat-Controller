// This requires parsing keys and mouse buttons from human readable names.

/*

use askama::Template;

#[derive(Template)]
#[template(path = "click.html")]
struct ClickTemplate {
    down_buffer: String,
    up_buffer: String,
}

pub async fn click(name: String) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let name = name.as_str();
    let down_buffer;

    if let Ok(button) = name.parse::<tfc::MouseButton>() {
        down_buffer = format!("{},{}", tfc::CommandCode::MouseClick as u8, button as u8);
    } else if let Ok(key) = name.parse::<tfc::Key>() {
        down_buffer = format!("{},{}", tfc::CommandCode::KeyClick as u8, key as u8);
    } else {
        return Ok(Box::new(warp::http::StatusCode::BAD_REQUEST))
    }

    Ok(Box::new(ClickTemplate {
        down_buffer,
        up_buffer: "".to_owned()
    }))
}

pub async fn downup(name: String) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let name = name.as_str();
    let down_buffer;
    let up_buffer;

    if let Ok(button) = name.parse::<tfc::MouseButton>() {
        down_buffer = format!("{},{}", tfc::CommandCode::MouseDown as u8, button as u8);
        up_buffer = format!("{},{}", tfc::CommandCode::MouseUp as u8, button as u8);
    } else if let Ok(key) = name.parse::<tfc::Key>() {
        down_buffer = format!("{},{}", tfc::CommandCode::KeyDown as u8, key as u8);
        up_buffer = format!("{},{}", tfc::CommandCode::KeyUp as u8, key as u8);
    } else {
        return Ok(Box::new(warp::http::StatusCode::BAD_REQUEST))
    }

    Ok(Box::new(ClickTemplate {
        down_buffer,
        up_buffer,
    }))
}

#[derive(Template)]
#[template(path = "press.html")]
struct PressTemplate {
    down_buffer: String,
    up_buffer: String,
}

pub async fn press(name: String) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let name = name.as_str();
    let down_buffer;
    let up_buffer;

    if let Ok(button) = name.parse::<tfc::MouseButton>() {
        down_buffer = format!("{},{}", tfc::CommandCode::MouseDown as u8, button as u8);
        up_buffer = format!("{},{}", tfc::CommandCode::MouseUp as u8, button as u8);
    } else if let Ok(key) = name.parse::<tfc::Key>() {
        down_buffer = format!("{},{}", tfc::CommandCode::KeyDown as u8, key as u8);
        up_buffer = format!("{},{}", tfc::CommandCode::KeyUp as u8, key as u8);
    } else {
        return Ok(Box::new(warp::http::StatusCode::BAD_REQUEST))
    }

    Ok(Box::new(PressTemplate {
        down_buffer,
        up_buffer,
    }))
}

*/
