use std::{fmt, fmt::Formatter};
use tfc::{Command, CommandCode, Key, MouseButton};

#[derive(Debug)]
pub enum ParseError<'a> {
    InvalidCommand(&'a str),
    InvalidKey(&'a str),
    InvalidMouseButton(&'a str),
    InvalidInteger(&'a str),
    MissingKey,
    MissingMouseButton,
    MissingInteger,
}

use ParseError::*;

impl<'a> std::fmt::Display for ParseError<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            InvalidCommand(s) => write!(f, "Expected command, found \"{}\"", s),
            InvalidKey(s) => write!(f, "Expected key, found \"{}\"", s),
            InvalidMouseButton(s) => write!(f, "Expected mouse button, found \"{}\"", s),
            InvalidInteger(s) => write!(f, "Expected integer, found \"{}\"", s),
            MissingKey => write!(f, "Expected key, reached end-of-input"),
            MissingMouseButton => write!(f, "Expected mouse button, reached end-of-input"),
            MissingInteger => write!(f, "Expected integer, reached end-of-input"),
        }
    }
}

impl<'a> std::error::Error for ParseError<'a> {}

fn parse_key<'a, I>(tokens: &mut I) -> Result<Key, ParseError<'a>>
    where I: std::iter::Iterator<Item = &'a str>
{
    let token = tokens.next().ok_or(MissingKey)?;
    token.parse::<Key>().map_err(|_| InvalidKey(token))
}

fn parse_mouse_button<'a, I>(tokens: &mut I) -> Result<MouseButton, ParseError<'a>>
    where I: std::iter::Iterator<Item = &'a str>
{
    let token = tokens.next().ok_or(MissingMouseButton)?;
    token.parse::<MouseButton>().map_err(|_| InvalidMouseButton(token))
}

fn parse_integer<'a, T, I>(tokens: &mut I) -> Result<T, ParseError<'a>>
    where T: std::str::FromStr,
          I: std::iter::Iterator<Item = &'a str>
{
    let token = tokens.next().ok_or(MissingInteger)?;
    token.parse::<T>().map_err(|_| InvalidInteger(token))
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
            Ok(MouseMoveRel) => Command::MouseMoveRel(parse_integer(&mut tokens)?, parse_integer(&mut tokens)?),
            Ok(MouseMoveAbs) => Command::MouseMoveAbs(parse_integer(&mut tokens)?, parse_integer(&mut tokens)?),
            Ok(MouseWarp) => Command::MouseWarp(parse_integer(&mut tokens)?, parse_integer(&mut tokens)?),
            Ok(MouseScroll) => Command::MouseScroll(parse_integer(&mut tokens)?, parse_integer(&mut tokens)?),
            Ok(MouseDown) => Command::MouseDown(parse_mouse_button(&mut tokens)?),
            Ok(MouseUp) => Command::MouseUp(parse_mouse_button(&mut tokens)?),
            Ok(MouseClick) => Command::MouseClick(parse_mouse_button(&mut tokens)?),
            Ok(Delay) => Command::Delay(parse_integer(&mut tokens)?),
            Err(_) => return Err(InvalidCommand(command_token)),
        });
    }

    Ok(commands)
}
