use lazy_static::lazy_static;
use tfc::{Command, CommandCode, Key, MouseButton, Enum};
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

fn get_lowercase_strings<E: Enum>() -> Vec<String> {
    let mut strings = Vec::new();
    strings.extend(E::iter().map(|v| v.identifier_name().to_ascii_lowercase()));
    strings
}

lazy_static! {
    static ref COMMAND_STRINGS: Vec<String> = get_lowercase_strings::<CommandCode>();
    static ref KEY_STRINGS: Vec<String> = get_lowercase_strings::<Key>();
    static ref MOUSE_BUTTON_STRINGS: Vec<String> = get_lowercase_strings::<MouseButton>();
}

trait Parsable<'a>: Sized {
    const MISSING: ParseError<'a>;
    fn parse(s: &'a str) -> Result<Self, ParseError<'a>>;
}

impl<'a> Parsable<'a> for CommandCode {
    // this MISSING constant is unused
    const MISSING: ParseError<'a> = MissingKey;
    fn parse(s: &'a str) -> Result<Self, ParseError<'a>> {
        match COMMAND_STRINGS.iter().position(|st| s == st) {
            Some(idx) => Ok(Self::from_u8(idx as u8).unwrap()),
            None => Err(InvalidCommand(s))
        }
    }
}

impl<'a> Parsable<'a> for Key {
    const MISSING: ParseError<'a> = MissingKey;
    fn parse(s: &'a str) -> Result<Self, ParseError<'a>> {
        match KEY_STRINGS.iter().position(|st| s == st) {
            Some(idx) => Ok(Self::from_u8(idx as u8).unwrap()),
            None => Err(InvalidKey(s))
        }
    }
}

impl<'a> Parsable<'a> for MouseButton {
    const MISSING: ParseError<'a> = MissingMouseButton;
    fn parse(s: &'a str) -> Result<Self, ParseError<'a>> {
        match KEY_STRINGS.iter().position(|st| s == st) {
            Some(idx) => Ok(Self::from_u8(idx as u8).unwrap()),
            None => Err(InvalidMouseButton(s))
        }
    }
}

impl<'a> Parsable<'a> for i32 {
    const MISSING: ParseError<'a> = MissingInteger;
    fn parse(s: &'a str) -> Result<Self, ParseError<'a>> {
        s.parse::<Self>().map_err(|_| InvalidInteger(s))
    }
}

impl<'a> Parsable<'a> for u32 {
    const MISSING: ParseError<'a> = MissingInteger;
    fn parse(s: &'a str) -> Result<Self, ParseError<'a>> {
        s.parse::<Self>().map_err(|_| InvalidInteger(s))
    }
}

fn parse<'a, T, I>(tokens: &mut I) -> Result<T, ParseError<'a>>
    where T: Parsable<'a>, I: Iterator<Item = &'a str>
{
    match tokens.next() {
        Some(t) => T::parse(t),
        None => T::parse("")
    }
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

        commands.push(match CommandCode::parse(command_token)? {
            Delay => Command::Delay(parse(&mut tokens)?),
            KeyDown => Command::KeyDown(parse(&mut tokens)?),
            KeyUp => Command::KeyUp(parse(&mut tokens)?),
            KeyClick => Command::KeyClick(parse(&mut tokens)?),
            MouseMoveRel => Command::MouseMoveRel(parse(&mut tokens)?, parse(&mut tokens)?),
            MouseMoveAbs => Command::MouseMoveAbs(parse(&mut tokens)?, parse(&mut tokens)?),
            MouseScroll => Command::MouseScroll(parse(&mut tokens)?, parse(&mut tokens)?),
            MouseDown => Command::MouseDown(parse(&mut tokens)?),
            MouseUp => Command::MouseUp(parse(&mut tokens)?),
            MouseClick => Command::MouseClick(parse(&mut tokens)?),
            // TODO: Extend this to handle the ASCII and Unicode commands
            _ => return Err(InvalidCommand(command_token)),
        });
    }

    Ok(commands)
}
