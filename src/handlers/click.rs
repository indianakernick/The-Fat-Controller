use askama::Template;
use crate::socket_command::{parse_mouse_button_name, parse_key_name, CommandCode};

#[derive(Template)]
#[template(path = "click.html")]
struct ClickTemplate {
    down_buffer: String,
    up_buffer: String,
}

pub async fn click(name: String) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let name = name.as_str();
    let down_buffer;

    if let Some(button) = parse_mouse_button_name(name) {
        down_buffer = format!("{},{}", CommandCode::MouseClick as u8, button as u8);
    } else if let Some(key) = parse_key_name(name) {
        down_buffer = format!("{},{}", CommandCode::KeyClick as u8, key as u8);
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

    if let Some(button) = parse_mouse_button_name(name) {
        down_buffer = format!("{},{}", CommandCode::MouseDown as u8, button as u8);
        up_buffer = format!("{},{}", CommandCode::MouseUp as u8, button as u8);
    } else if let Some(key) = parse_key_name(name) {
        down_buffer = format!("{},{}", CommandCode::KeyDown as u8, key as u8);
        up_buffer = format!("{},{}", CommandCode::KeyUp as u8, key as u8);
    } else {
        return Ok(Box::new(warp::http::StatusCode::BAD_REQUEST))
    }

    Ok(Box::new(ClickTemplate {
        down_buffer,
        up_buffer,
    }))
}
