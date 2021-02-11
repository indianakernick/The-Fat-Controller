use std::{fmt, fmt::Formatter};
use tfc::{Command, CommandCode, Key, MouseButton};

#[derive(Debug)]
pub enum ParseError<'a> {
    InvalidCommand(&'a str),
    InvalidKey(&'a str),
    InvalidMouseButton(&'a str),
    MissingKey,
    MissingMouseButton,
}

use ParseError::*;

impl<'a> std::fmt::Display for ParseError<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            InvalidCommand(s) => write!(f, "Expected command, found \"{}\"", s),
            InvalidKey(s) => write!(f, "Expected key, found \"{}\"", s),
            InvalidMouseButton(s) => write!(f, "Expected mouse button, found \"{}\"", s),
            MissingKey => write!(f, "Expected key, reached end-of-input"),
            MissingMouseButton => write!(f, "Expected mouse button, reached end-of-input"),
        }
    }
}

impl<'a> std::error::Error for ParseError<'a> {}

fn parse_key<'a, I>(tokens: &mut I) -> Result<Key, ParseError<'a>>
    where I: std::iter::Iterator<Item = &'a str>
{
    let key_token = match tokens.next() {
        Some(token) => token,
        None => return Err(MissingKey),
    };
    match key_token.parse::<Key>() {
        Ok(key) => Ok(key),
        Err(_) => Err(InvalidKey(key_token)),
    }
}

fn parse_mouse_button<'a, I>(tokens: &mut I) -> Result<MouseButton, ParseError<'a>>
    where I: std::iter::Iterator<Item = &'a str>
{
    let button_token = match tokens.next() {
        Some(token) => token,
        None => return Err(MissingMouseButton),
    };
    match button_token.parse::<MouseButton>() {
        Ok(button) => Ok(button),
        Err(_) => Err(InvalidMouseButton(button_token)),
    }
}

pub fn parse_tokens<'a, I>(mut tokens: I) -> Result<Vec<Command>, ParseError<'a>>
    where I: std::iter::Iterator<Item = &'a str>
{
    let mut commands = Vec::new();

    loop {
        use CommandCode::*;

        let command_token = match tokens.next() {
            Some(token) => token,
            None => break,
        };

        commands.push(match command_token.parse::<CommandCode>() {
            Ok(KeyDown) => Command::KeyDown(parse_key(&mut tokens)?),
            Ok(KeyUp) => Command::KeyUp(parse_key(&mut tokens)?),
            Ok(KeyClick) => Command::KeyClick(parse_key(&mut tokens)?),
            Ok(MouseMoveRel) => Command::Delay(0),
            Ok(MouseMoveAbs) => Command::Delay(0),
            Ok(MouseWarp) => Command::Delay(0),
            Ok(MouseScroll) => Command::Delay(0),
            Ok(MouseDown) => Command::MouseDown(parse_mouse_button(&mut tokens)?),
            Ok(MouseUp) => Command::MouseUp(parse_mouse_button(&mut tokens)?),
            Ok(MouseClick) => Command::MouseClick(parse_mouse_button(&mut tokens)?),
            Ok(Delay) => Command::Delay(0),
            Err(_) => return Err(InvalidCommand(command_token)),
        });
    }

    Ok(commands)
}
