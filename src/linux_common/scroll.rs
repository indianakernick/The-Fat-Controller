const THRESHOLD: i32 = 120;

pub struct ScrollAccum {
    x: i32,
    y: i32,
}

impl Default for ScrollAccum {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
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
