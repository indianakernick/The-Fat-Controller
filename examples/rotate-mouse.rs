use tfc::{Context, Error, traits::*};
use std::{f64::consts::PI, thread, time::Duration};

fn main() -> Result<(), Error> {
    let mut ctx = Context::new()?;
    let radius = 100.0;
    let center = ctx.cursor_location()?;
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