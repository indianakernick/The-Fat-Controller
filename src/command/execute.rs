use super::Command;
use std::{thread, time::Duration};
use crate::{GenericError, traits::*};

impl Command {
    fn execute_core<C>(&self, ctx: &mut C) -> Result<bool, GenericError<C::PlatformError>>
        where C: FallibleContext + KeyboardContext + MouseContext + AsciiKeyboardContext
    {
        use Command::*;
        match self {
            KeyDown(key) => ctx.key_down(*key),
            KeyUp(key) => ctx.key_up(*key),
            KeyClick(key) => ctx.key_click(*key),
            MouseMoveRel(dx, dy) => ctx.mouse_move_rel(*dx, *dy),
            MouseMoveAbs(x, y) => ctx.mouse_move_abs(*x, *y),
            MouseScroll(dx, dy) => ctx.mouse_scroll(*dx, *dy),
            MouseDown(button) => ctx.mouse_down(*button),
            MouseUp(button) => ctx.mouse_up(*button),
            MouseClick(button) => ctx.mouse_click(*button),
            AsciiCharDown(ch) => ctx.ascii_char_down(*ch),
            AsciiCharUp(ch) => ctx.ascii_char_up(*ch),
            AsciiChar(ch) => ctx.ascii_char(*ch),
            AsciiString(s) => ctx.ascii_string(s.as_slice()),
            _ => return Ok(false)
        }?;
        Ok(true)
    }

    #[cfg(not(all(
        not(feature = "ascii-fallback"),
        target_os = "linux",
        not(x11)
    )))]
    fn execute_unicode<C>(&self, ctx: &mut C) -> Result<bool, GenericError<C::PlatformError>>
        where C: FallibleContext + KeyboardContext + MouseContext + AsciiKeyboardContext + UnicodeKeyboardContext
    {
        if self.execute_core(ctx)? {
            return Ok(true);
        }

        use Command::*;
        match self {
            UnicodeCharDown(ch) => ctx.unicode_char_down(*ch),
            UnicodeCharUp(ch) => ctx.unicode_char_up(*ch),
            UnicodeChar(ch) => ctx.unicode_char(*ch),
            UnicodeString(s) => ctx.unicode_string(s.as_str()),
            _ => return Ok(false)
        }?;

        Ok(true)
    }

    #[cfg(all(
        not(feature = "ascii-fallback"),
        target_os = "linux",
        not(x11)
    ))]
    pub fn execute_unicode<C>(&self, ctx: &mut C) -> Result<(), GenericError<C::PlatformError>>
        where C: FallibleContext + KeyboardContext + MouseContext + AsciiKeyboardContext
    {
        if self.execute_core(ctx)? {
            return Ok(true);
        }

        use Command::*;
        match self {
            UnicodeCharDown(_) => panic!("UnicodeKeyboardContext is not implemented"),
            UnicodeCharUp(_) => panic!("UnicodeKeyboardContext is not implemented"),
            UnicodeChar(_) => panic!("UnicodeKeyboardContext is not implemented"),
            UnicodeString(_) => panic!("UnicodeKeyboardContext is not implemented"),
            _ => return Ok(false)
        }?;

        Ok(true)
    }

    /// Execute a [`Command`] by calling the corresponding method on
    /// one of the [`traits`](crate::traits).
    ///
    /// [`Delay`](Command::Delay) is treated as being synchronous and is
    /// executed using [`std::thread::sleep`].
    #[cfg(not(all(
        not(feature = "ascii-fallback"),
        target_os = "linux",
        not(x11)
    )))]
    pub fn execute<C>(&self, ctx: &mut C) -> Result<(), GenericError<C::PlatformError>>
        where C: FallibleContext + KeyboardContext + MouseContext + AsciiKeyboardContext + UnicodeKeyboardContext
    {
        if self.execute_unicode(ctx)? {
            return Ok(());
        }

        use Command::*;
        match self {
            Delay(millis) => Ok(thread::sleep(Duration::from_millis(*millis as u64))),
            _ => std::unreachable!()
        }
    }

    /// Execute a [`Command`] by calling the corresponding method on
    /// one of the [`traits`](crate::traits).
    ///
    /// [`Delay`](Command::Delay) is treated as being asynchronous and is
    /// executed using [`tokio::time::sleep`].
    #[cfg(all(
        not(all(
            not(feature = "ascii-fallback"),
            target_os = "linux",
            not(x11)
        )),
        feature = "tokio"
    ))]
    pub async fn execute_async<C>(&self, ctx: &mut C) -> Result<(), GenericError<C::PlatformError>>
        where C: FallibleContext + KeyboardContext + MouseContext + AsciiKeyboardContext + UnicodeKeyboardContext
    {
        if self.execute_core(ctx)? {
            return Ok(());
        }

        use Command::*;
        match self {
            Delay(millis) => Ok(tokio::time::sleep(Duration::from_millis(*millis as u64)).await),
            _ => std::unreachable!()
        }
    }

    /// Execute a [`Command`] by calling the corresponding method on
    /// one of the [`traits`](crate::traits).
    ///
    /// [`Delay`](Command::Delay) is treated as being synchronous and is
    /// executed using [`std::thread::sleep`].
    #[cfg(all(
        not(feature = "ascii-fallback"),
        target_os = "linux",
        not(x11)
    ))]
    pub fn execute<C>(&self, ctx: &mut C) -> Result<(), GenericError<C::PlatformError>>
        where C: FallibleContext + KeyboardContext + MouseContext + AsciiKeyboardContext
    {
        if self.execute_unicode(ctx)? {
            return Ok(());
        }

        use Command::*;
        match self {
            Delay(millis) => Ok(thread::sleep(Duration::from_millis(*millis as u64))),
            _ => std::unreachable!()
        }
    }

    /// Execute a [`Command`] by calling the corresponding method on
    /// one of the [`traits`](crate::traits).
    ///
    /// [`Delay`](Command::Delay) is treated as being asynchronous and is
    /// executed using [`tokio::time::sleep`].
    #[cfg(all(
        not(feature = "ascii-fallback"),
        target_os = "linux",
        not(x11),
        feature = "tokio"
    ))]
    pub async fn execute_async<C>(&self, ctx: &mut C) -> Result<(), GenericError<C::PlatformError>>
        where C: FallibleContext + KeyboardContext + MouseContext + AsciiKeyboardContext
    {
        if self.execute_unicode(ctx)? {
            return Ok(());
        }

        use Command::*;
        match self {
            Delay(millis) => Ok(tokio::time::sleep(Duration::from_millis(*millis as u64)).await),
            _ => std::unreachable!()
        }
    }
}