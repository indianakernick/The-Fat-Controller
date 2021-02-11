use tfc::{Command, CommandCode, Key, MouseButton};
use std::{fmt::{self, Display, Formatter}, iter::Iterator};

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

impl<'a> Display for ParseError<'a> {
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

trait Parsable<'a>: std::str::FromStr {
    const MISSING: ParseError<'a> = MissingInteger;
    fn invalid(s: &'a str) -> ParseError<'a> {
        InvalidInteger(s)
    }
}

impl<'a> Parsable<'a> for Key {
    const MISSING: ParseError<'a> = MissingKey;
    fn invalid(s: &'a str) -> ParseError<'a> {
        InvalidKey(s)
    }
}

impl<'a> Parsable<'a> for MouseButton {
    const MISSING: ParseError<'a> = MissingMouseButton;
    fn invalid(s: &'a str) -> ParseError<'a> {
        InvalidMouseButton(s)
    }
}

impl<'a> Parsable<'a> for i32 {}
impl<'a> Parsable<'a> for u32 {}

fn parse<'a, T, I>(tokens: &mut I) -> Result<T, ParseError<'a>>
    where T: Parsable<'a>, I: Iterator<Item = &'a str>
{
    let token = tokens.next().ok_or(T::MISSING)?;
    token.parse::<T>().map_err(|_| T::invalid(token))
}

pub fn parse_tokens<'a, I>(mut tokens: I) -> Result<Vec<Command>, ParseError<'a>>
    where I: Iterator<Item = &'a str>
{
    let mut commands = Vec::new();

    loop {
        use CommandCode::*;

        let command_token = match tokens.next() {
            Some(token) => token,
            None => break,
        };

        commands.push(match command_token.parse::<CommandCode>() {
            Ok(KeyDown) => Command::KeyDown(parse(&mut tokens)?),
            Ok(KeyUp) => Command::KeyUp(parse(&mut tokens)?),
            Ok(KeyClick) => Command::KeyClick(parse(&mut tokens)?),
            Ok(MouseMoveRel) => Command::MouseMoveRel(parse(&mut tokens)?, parse(&mut tokens)?),
            Ok(MouseMoveAbs) => Command::MouseMoveAbs(parse(&mut tokens)?, parse(&mut tokens)?),
            Ok(MouseWarp) => Command::MouseWarp(parse(&mut tokens)?, parse(&mut tokens)?),
            Ok(MouseScroll) => Command::MouseScroll(parse(&mut tokens)?, parse(&mut tokens)?),
            Ok(MouseDown) => Command::MouseDown(parse(&mut tokens)?),
            Ok(MouseUp) => Command::MouseUp(parse(&mut tokens)?),
            Ok(MouseClick) => Command::MouseClick(parse(&mut tokens)?),
            Ok(Delay) => Command::Delay(parse(&mut tokens)?),
            Err(_) => return Err(InvalidCommand(command_token)),
        });
    }

    Ok(commands)
}
