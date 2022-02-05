use std::{thread, time::Duration};
use tfc::{Context, Error, traits::*};

// `unicode_char('s')` should always type an `s` no matter what the keyboard
// layout is. `ascii_char(b's')` will press the key in the position of an `s` on
// a QWERTY keyboard. When using the QWERTY layout, this results in an `s` being
// typed. However, when using a different keyboard layout, pressing the `s` key
// might result in a different character being typed. For example, using Dvorak
// would result in an `o` being typed if the `s` key is pressed.

fn main() -> Result<(), Error> {
    let delay = Duration::from_millis(50);

    let mut ctx = Context::new()?;

    for c in b' '..=b'~' {
        thread::sleep(delay);
        ctx.unicode_char(c as char)?;
        ctx.ascii_char(b' ')?;
        ctx.ascii_char(c)?;
        ctx.ascii_char(b'\n')?;
    }

    Ok(())
}
