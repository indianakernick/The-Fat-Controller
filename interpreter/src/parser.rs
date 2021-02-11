use std::{fmt, fmt::Formatter};
use tfc::{Command, CommandCode, Key, MouseButton};

#[derive(Debug)]
pub enum ParseError<'a> {
    InvalidCommand(&'a str),
}

use ParseError::*;

impl<'a> std::fmt::Display for ParseError<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            InvalidCommand(s) => write!(f, "Expected command, found \"{}\"", s),
        }
    }
}

impl<'a> std::error::Error for ParseError<'a> {}

pub fn parse_tokens<'a, I: std::iter::Iterator<Item = &'a str>>(tokens: I) -> Result<Vec<Command>, ParseError<'a>> {
    let mut commands = Vec::new();

    for token in tokens {
        match token.parse::<CommandCode>() {
            Ok(CommandCode::KeyDown) => {},
            Ok(CommandCode::KeyUp) => {},
            Ok(CommandCode::KeyClick) => {},
            Ok(CommandCode::MouseMoveRel) => {},
            Ok(CommandCode::MouseMoveAbs) => {},
            Ok(CommandCode::MouseWarp) => {},
            Ok(CommandCode::MouseScroll) => {},
            Ok(CommandCode::MouseDown) => {},
            Ok(CommandCode::MouseUp) => {},
            Ok(CommandCode::MouseClick) => {},
            Ok(CommandCode::Delay) => {},
            Err(_) => return Err(InvalidCommand(token)),
        }
    }

    Ok(commands)
}
