const THRESHOLD: i32 = 120;

#[derive(Default)]
pub struct ScrollAccum {
    x: i32,
    y: i32,
}

impl ScrollAccum {
    pub fn accumulate(&mut self, x: i32, y: i32) -> (i32, i32) {
        self.x += x;
        self.y += y;
        let result = (self.x / THRESHOLD, self.y / THRESHOLD);
        self.x %= THRESHOLD;
        self.y %= THRESHOLD;
        result
    }
}
