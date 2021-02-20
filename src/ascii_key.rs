use crate::{Error, Key, KeyboardContext};

/// A pair that holds a [`Key`](Key) and an indication of whether the shift key
/// must be pressed.
///
/// An instance of this struct can be created from an ASCII character and can
/// later be applied to a [`KeyboardContext`](KeyboardContext). This assumes a
/// standard US keyboard layout and is unlikely to work correctly otherwise.
///
/// This is meant to be a fallback for platforms that don't have
/// [`UnicodeKeyboardContext`](crate::UnicodeKeyboardContext) (i.e.
/// Linux-Wayland). This may also be used for performance reasons as it may be
/// slightly faster than
/// [`UnicodeKeyboardContext`](crate::UnicodeKeyboardContext). That's if you're
/// willing to accept ASCII on a standard US keyboard layout.
#[repr(packed)] // This doesn't pack it into one byte. Maybe it will one day?
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AsciiKey {
    pub key: Key,
    pub shift: bool,
}

impl AsciiKey {
    fn new(key: Key) -> Self {
        Self { key, shift: false }
    }

    fn new_shift(key: Key) -> Self {
        Self { key, shift: true }
    }

    /// Construct an [`AsciiKey`](AsciiKey) from an ASCII character.
    ///
    /// This function only works with ASCII characters and will return `None` if
    /// an unsupported character is given.
    ///
    /// All printable characters are translated as you would expect. Some
    /// special characters are also handled.
    ///
    /// | ASCII              | Key                      |
    /// | ------------------ | ------------------------ |
    /// | `0x08` (backspace) | `Key::DeleteOrBackspace` |
    /// | `0x09` (tab)       | `Key::Tab`               |
    /// | `0x0A` (linefeed)  | `Key::ReturnOrEnter`     |
    /// | `0x1B` (escape)    | `Key::Escape`            |
    /// | `0x7F` (delete)    | `Key::DeleteOrBackspace` |
    ///
    /// # Example
    ///
    /// ```
    /// use tfc::{AsciiKey, Key};
    ///
    /// let tilde = AsciiKey::from_ascii('~').unwrap();
    /// assert_eq!(tilde.key, Key::Grave);
    /// assert!(tilde.shift);
    ///
    /// let lowercase_t = AsciiKey::from_ascii('t').unwrap();
    /// assert_eq!(lowercase_t.key, Key::T);
    /// assert!(!lowercase_t.shift);
    ///
    /// assert!(AsciiKey::from_ascii('ã').is_none());
    /// ```
    pub fn from_ascii(ascii: char) -> Option<Self> {
        use Key::*;
        match ascii {
            '\x08' | '\x7F' => Some(Self::new(DeleteOrBackspace)),
            '\t' => Some(Self::new(Tab)),
            '\n' => Some(Self::new(ReturnOrEnter)),
            '\x1B' => Some(Self::new(Escape)),

            ' ' => Some(Self::new(Space)),
            '!' => Some(Self::new_shift(N1)),
            '"' => Some(Self::new_shift(Quote)),
            '#' => Some(Self::new_shift(N3)),
            '$' => Some(Self::new_shift(N4)),
            '%' => Some(Self::new_shift(N5)),
            '&' => Some(Self::new_shift(N7)),
            '\'' => Some(Self::new(Quote)),
            '(' => Some(Self::new_shift(N9)),
            ')' => Some(Self::new_shift(N0)),
            '*' => Some(Self::new_shift(N8)),
            '+' => Some(Self::new_shift(Equal)),
            ',' => Some(Self::new(Comma)),
            '-' => Some(Self::new(Minus)),
            '.' => Some(Self::new(Period)),
            '/' => Some(Self::new(Slash)),

            '0' => Some(Self::new(N0)),
            '1' => Some(Self::new(N1)),
            '2' => Some(Self::new(N2)),
            '3' => Some(Self::new(N3)),
            '4' => Some(Self::new(N4)),
            '5' => Some(Self::new(N5)),
            '6' => Some(Self::new(N6)),
            '7' => Some(Self::new(N7)),
            '8' => Some(Self::new(N8)),
            '9' => Some(Self::new(N9)),

            ':' => Some(Self::new_shift(Semicolon)),
            ';' => Some(Self::new(Semicolon)),
            '<' => Some(Self::new_shift(Comma)),
            '=' => Some(Self::new(Equal)),
            '>' => Some(Self::new_shift(Period)),
            '?' => Some(Self::new_shift(Slash)),
            '@' => Some(Self::new_shift(N2)),

            'A' => Some(Self::new_shift(A)),
            'B' => Some(Self::new_shift(B)),
            'C' => Some(Self::new_shift(C)),
            'D' => Some(Self::new_shift(D)),
            'E' => Some(Self::new_shift(E)),
            'F' => Some(Self::new_shift(F)),
            'G' => Some(Self::new_shift(G)),
            'H' => Some(Self::new_shift(H)),
            'I' => Some(Self::new_shift(I)),
            'J' => Some(Self::new_shift(J)),
            'K' => Some(Self::new_shift(K)),
            'L' => Some(Self::new_shift(L)),
            'M' => Some(Self::new_shift(M)),
            'N' => Some(Self::new_shift(N)),
            'O' => Some(Self::new_shift(O)),
            'P' => Some(Self::new_shift(P)),
            'Q' => Some(Self::new_shift(Q)),
            'R' => Some(Self::new_shift(R)),
            'S' => Some(Self::new_shift(S)),
            'T' => Some(Self::new_shift(T)),
            'U' => Some(Self::new_shift(U)),
            'V' => Some(Self::new_shift(V)),
            'W' => Some(Self::new_shift(W)),
            'X' => Some(Self::new_shift(X)),
            'Y' => Some(Self::new_shift(Y)),
            'Z' => Some(Self::new_shift(Z)),

            '[' => Some(Self::new(LeftBracket)),
            '\\' => Some(Self::new(Backslash)),
            ']' => Some(Self::new(RightBracket)),
            '^' => Some(Self::new_shift(N6)),
            '_' => Some(Self::new_shift(Minus)),
            '`' => Some(Self::new(Grave)),

            'a' => Some(Self::new(A)),
            'b' => Some(Self::new(B)),
            'c' => Some(Self::new(C)),
            'd' => Some(Self::new(D)),
            'e' => Some(Self::new(E)),
            'f' => Some(Self::new(F)),
            'g' => Some(Self::new(G)),
            'h' => Some(Self::new(H)),
            'i' => Some(Self::new(I)),
            'j' => Some(Self::new(J)),
            'k' => Some(Self::new(K)),
            'l' => Some(Self::new(L)),
            'm' => Some(Self::new(M)),
            'n' => Some(Self::new(N)),
            'o' => Some(Self::new(O)),
            'p' => Some(Self::new(P)),
            'q' => Some(Self::new(Q)),
            'r' => Some(Self::new(R)),
            's' => Some(Self::new(S)),
            't' => Some(Self::new(T)),
            'u' => Some(Self::new(U)),
            'v' => Some(Self::new(V)),
            'w' => Some(Self::new(W)),
            'x' => Some(Self::new(X)),
            'y' => Some(Self::new(Y)),
            'z' => Some(Self::new(Z)),

            '{' => Some(Self::new_shift(LeftBracket)),
            '|' => Some(Self::new_shift(Backslash)),
            '}' => Some(Self::new_shift(RightBracket)),
            '~' => Some(Self::new_shift(Grave)),
            // Delete is handled above

            _ => None,
        }
    }

    /// Apply an [`AsciiKey`](AsciiKey) to a
    /// [`KeyboardContext`](KeyboardContext).
    ///
    /// This will press and release the key while also pressing and releasing
    /// shift if necessary.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use tfc::{AsciiKey, Context};
    ///
    /// let mut context = Context::new().unwrap();
    /// let dollar = AsciiKey::from_ascii('$').unwrap();
    /// dollar.apply(&mut context).unwrap();
    /// ```
    pub fn apply<C: KeyboardContext>(&self, ctx: &mut C) -> Result<(), Error> {
        if self.shift {
            ctx.key_down(Key::Shift)?;
            ctx.key_click(self.key)?;
            ctx.key_up(Key::Shift)
        } else {
            ctx.key_click(self.key)
        }
    }

    /// A convenience function that constructs and applies an
    /// [`AsciiKey`](AsciiKey) from an ASCII character.
    ///
    /// Returns `None` if the given character is unsupported.
    pub fn apply_ascii_char<C>(ctx: &mut C, ascii: char) -> Option<Result<(), Error>>
        where C: KeyboardContext
    {
        Some(Self::from_ascii(ascii)?.apply(ctx))
    }

    /// A convenience function that constructs and applies an
    /// [`AsciiKey`](AsciiKey) from an ASCII string.
    ///
    /// If any of the characters in the string are unsupported, `None` will be
    /// returned and no key presses will occur.
    pub fn apply_ascii_string<C>(ctx: &mut C, ascii: &str) -> Option<Result<(), Error>>
        where C: KeyboardContext
    {
        for ch in ascii.bytes() {
            if Self::from_ascii(ch as char).is_none() {
                return None;
            }
        }

        for ch in ascii.bytes() {
            if let Err(e) = Self::from_ascii(ch as char).unwrap().apply(ctx) {
                return Some(Err(e));
            }
        }

        Some(Ok(()))
    }
}
