use crate::{Error, Key, KeyboardContext};

/// A context that supports layout-dependent ASCII keyboard events.
///
/// Internally, this will map ASCII characters to [`Key`](Key)s which means that
/// a **standard US keyboard layout** is assumed. Using this with other keyboard
/// layouts is unlikely to produce the desired results.
///
/// This is meant to be a fallback for platforms that don't have
/// [`UnicodeKeyboardContext`](crate::UnicodeKeyboardContext) (i.e.
/// Linux-Wayland). This may also be used for performance reasons as it may be
/// slightly faster than
/// [`UnicodeKeyboardContext`](crate::UnicodeKeyboardContext). That's if you're
/// willing to accept ASCII on a standard US keyboard layout.
///
/// All printable characters are translated as you would expect and some special
/// characters are also handled.
///
/// | ASCII              | Key                      |
/// | ------------------ | ------------------------ |
/// | `0x08` (backspace) | `Key::DeleteOrBackspace` |
/// | `0x09` (tab)       | `Key::Tab`               |
/// | `0x0A` (linefeed)  | `Key::ReturnOrEnter`     |
/// | `0x1B` (escape)    | `Key::Escape`            |
/// | `0x7F` (delete)    | `Key::DeleteOrBackspace` |
pub trait AsciiKeyboardContext {
    /// Generate a key press and release event to type an ASCII character.
    ///
    /// Returns `None` if the given character is unsupported.
    ///
    /// # Arguments
    ///
    /// * `ch` - The ASCII character to type.
    fn ascii_char(&mut self, ch: u8) -> Option<Result<(), Error>>;

    /// Generate key presses and releases such that an ASCII string is typed.
    ///
    /// If any of the characters in the string are unsupported, `None` will be
    /// returned and no key presses will occur.
    ///
    /// # Arguments
    ///
    /// * `s` - The ASCII string to type.
    fn ascii_string(&mut self, s: &[u8]) -> Option<Result<(), Error>>;
}

// Essentially an Option<(bool, Key)> packed into a single byte.
// As of writing this, the compiler will optimize nested enums but not adjacent
// enums. So Option<(bool, Key)> is 2 bytes when it could be 1.
#[derive(Eq, PartialEq)]
#[repr(transparent)]
struct KeyShift(u8);

// The most significant bit is used for the shift state. The remaining 7 bits
// are for the key. The all-ones bit pattern is reserved for NONE. We use this
// static assertion to ensure that Key will fit.
const _: [u8; 1] = [0; (Key::COUNT < 127) as usize];

impl KeyShift {
    const NONE: Self = Self(255);

    fn new(key: Key) -> Self {
        Self(key as u8)
    }

    fn new_shift(key: Key) -> Self {
        Self(key as u8 | 128)
    }

    fn shift(&self) -> bool {
        self.0 & 128 != 0
    }

    fn key(&self) -> Key {
        // Constructing from new or new_shift guarantees that this is safe.
        unsafe {
            std::mem::transmute(self.0 & 127)
        }
    }

    fn from_ascii(ascii: u8) -> Self {
        use Key::*;
        match ascii {
            0x08 | 0x7F => Self::new(DeleteOrBackspace),
            b'\t' => Self::new(Tab),
            b'\n' => Self::new(ReturnOrEnter),
            0x1B => Self::new(Escape),

            b' ' => Self::new(Space),
            b'!' => Self::new_shift(N1),
            b'"' => Self::new_shift(Quote),
            b'#' => Self::new_shift(N3),
            b'$' => Self::new_shift(N4),
            b'%' => Self::new_shift(N5),
            b'&' => Self::new_shift(N7),
            b'\'' => Self::new(Quote),
            b'(' => Self::new_shift(N9),
            b')' => Self::new_shift(N0),
            b'*' => Self::new_shift(N8),
            b'+' => Self::new_shift(Equal),
            b',' => Self::new(Comma),
            b'-' => Self::new(Minus),
            b'.' => Self::new(Period),
            b'/' => Self::new(Slash),

            b'0' => Self::new(N0),
            b'1' => Self::new(N1),
            b'2' => Self::new(N2),
            b'3' => Self::new(N3),
            b'4' => Self::new(N4),
            b'5' => Self::new(N5),
            b'6' => Self::new(N6),
            b'7' => Self::new(N7),
            b'8' => Self::new(N8),
            b'9' => Self::new(N9),

            b':' => Self::new_shift(Semicolon),
            b';' => Self::new(Semicolon),
            b'<' => Self::new_shift(Comma),
            b'=' => Self::new(Equal),
            b'>' => Self::new_shift(Period),
            b'?' => Self::new_shift(Slash),
            b'@' => Self::new_shift(N2),

            b'A' => Self::new_shift(A),
            b'B' => Self::new_shift(B),
            b'C' => Self::new_shift(C),
            b'D' => Self::new_shift(D),
            b'E' => Self::new_shift(E),
            b'F' => Self::new_shift(F),
            b'G' => Self::new_shift(G),
            b'H' => Self::new_shift(H),
            b'I' => Self::new_shift(I),
            b'J' => Self::new_shift(J),
            b'K' => Self::new_shift(K),
            b'L' => Self::new_shift(L),
            b'M' => Self::new_shift(M),
            b'N' => Self::new_shift(N),
            b'O' => Self::new_shift(O),
            b'P' => Self::new_shift(P),
            b'Q' => Self::new_shift(Q),
            b'R' => Self::new_shift(R),
            b'S' => Self::new_shift(S),
            b'T' => Self::new_shift(T),
            b'U' => Self::new_shift(U),
            b'V' => Self::new_shift(V),
            b'W' => Self::new_shift(W),
            b'X' => Self::new_shift(X),
            b'Y' => Self::new_shift(Y),
            b'Z' => Self::new_shift(Z),

            b'[' => Self::new(LeftBracket),
            b'\\' => Self::new(Backslash),
            b']' => Self::new(RightBracket),
            b'^' => Self::new_shift(N6),
            b'_' => Self::new_shift(Minus),
            b'`' => Self::new(Grave),

            b'a' => Self::new(A),
            b'b' => Self::new(B),
            b'c' => Self::new(C),
            b'd' => Self::new(D),
            b'e' => Self::new(E),
            b'f' => Self::new(F),
            b'g' => Self::new(G),
            b'h' => Self::new(H),
            b'i' => Self::new(I),
            b'j' => Self::new(J),
            b'k' => Self::new(K),
            b'l' => Self::new(L),
            b'm' => Self::new(M),
            b'n' => Self::new(N),
            b'o' => Self::new(O),
            b'p' => Self::new(P),
            b'q' => Self::new(Q),
            b'r' => Self::new(R),
            b's' => Self::new(S),
            b't' => Self::new(T),
            b'u' => Self::new(U),
            b'v' => Self::new(V),
            b'w' => Self::new(W),
            b'x' => Self::new(X),
            b'y' => Self::new(Y),
            b'z' => Self::new(Z),

            b'{' => Self::new_shift(LeftBracket),
            b'|' => Self::new_shift(Backslash),
            b'}' => Self::new_shift(RightBracket),
            b'~' => Self::new_shift(Grave),
            // Delete is handled above

            _ => Self::NONE,
        }
    }
}

fn apply<C: KeyboardContext>(ctx: &mut C, key_shift: KeyShift) -> Result<(), Error> {
    if key_shift.shift() {
        ctx.key_down(Key::Shift)?;
        ctx.key_click(key_shift.key())?;
        ctx.key_up(Key::Shift)
    } else {
        ctx.key_click(key_shift.key())
    }
}

impl<C: KeyboardContext> AsciiKeyboardContext for C {
    fn ascii_char(&mut self, ch: u8) -> Option<Result<(), Error>> {
        let key_shift = KeyShift::from_ascii(ch);
        if key_shift == KeyShift::NONE {
            return None;
        }
        Some(apply(self, key_shift))
    }

    fn ascii_string(&mut self, s: &[u8]) -> Option<Result<(), Error>> {
        for ch in s.iter() {
            if KeyShift::from_ascii(*ch) == KeyShift::NONE {
                return None;
            }
        }

        for ch in s.iter() {
            if let Err(e) = apply(self, KeyShift::from_ascii(*ch)) {
                return Some(Err(e));
            }
        }

        Some(Ok(()))
    }
}
