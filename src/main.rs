mod filters;
mod handlers;

use warp::Filter;
use tokio::sync::mpsc;
use enigo::{Enigo, Key, MouseButton, KeyboardControllable, MouseControllable};

pub enum EnigoCommand {
    MouseMoveTo(i32, i32),
    MouseMoveRelative(i32, i32),
    MouseDown(MouseButton),
    MouseUp(MouseButton),
    MouseClick(MouseButton),
    MouseScrollX(i32),
    MouseScrollY(i32),

    KeyDown(Key),
    KeyUp(Key),
    KeyClick(Key),
}

fn parse_command(enigo: &mut Enigo, command: EnigoCommand) {
    use EnigoCommand::*;
    match command {
        MouseMoveTo(x, y) => enigo.mouse_move_to(x, y),
        MouseMoveRelative(x, y) => enigo.mouse_move_relative(x, y),
        MouseDown(button) => enigo.mouse_down(button),
        MouseUp(button) => enigo.mouse_up(button),
        MouseClick(button) => enigo.mouse_click(button),
        MouseScrollX(length) => enigo.mouse_scroll_x(length),
        MouseScrollY(length) => enigo.mouse_scroll_y(length),

        KeyDown(key) => enigo.key_down(key),
        KeyUp(key) => enigo.key_up(key),
        KeyClick(key) => enigo.key_click(key),
    }
}

#[tokio::main(flavor="current_thread")]
async fn main() {
    let mut enigo = Enigo::new();
    let (ch_tx, mut ch_rx) = mpsc::unbounded_channel::<EnigoCommand>();

    let ctx = handlers::SocketContext::new(ch_tx);

    pretty_env_logger::init();

    let routes = filters::click()
        .or(filters::socket(ctx))
        .or(filters::js())
        .or(filters::css());

    tokio::spawn(async {
        warp::serve(routes.with(warp::log("key")))
            .run(([0, 0, 0, 0], 80))
            .await;
    });

    while let Some(command) = ch_rx.recv().await {
        //let start = std::time::SystemTime::now();
        //println!("Recv command {}", start.duration_since(std::time::UNIX_EPOCH).unwrap().as_micros());
        parse_command(&mut enigo, command);
        //let end = std::time::SystemTime::now();
        //println!("Exec command {}", end.duration_since(start).unwrap().as_micros());
    }
}
