<img alt="Sir Topham Hatt (The Fat Controller) from Thomas the Tank Engine" width="200" align="left" src="https://upload.wikimedia.org/wikipedia/en/f/fc/Sir_Topham_Hatt_1986.jpg"/>

# The Fat Controller

TFC is a library for simulating mouse and keyboard events. Mouse movement, mouse
clicking, scrolling and key presses can all be simulated. Currently, the library only supports macOS but other platforms may be added in
the future (perhaps with the help of some contributors 😉).

## Usage

Add the following to your `Cargo.toml`:

```toml
[dependencies]
tfc = "0.1"
```

## Example

```rust
use tfc::*;
use std::{thread, time::Duration, f64::consts::PI};

fn main() -> Result<(), Error> {
    let mut ctx = Context::new()?;
    let radius = 100.0;
    let center = ctx.mouse_location()?;
    let center = (center.0 as f64 - radius, center.1 as f64);
    let steps = 200;
    let revolutions = 3;
    let delay = Duration::from_millis(10);

    for step in 0..steps * revolutions {
        thread::sleep(delay);
        let angle = step as f64 * 2.0 * PI / steps as f64;
        let x = (center.0 + radius * angle.cos()).round() as i32;
        let y = (center.1 + radius * angle.sin()).round() as i32;
        ctx.mouse_move_abs(x, y)?;
    }

    Ok(())
}
```
