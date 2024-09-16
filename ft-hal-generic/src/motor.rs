pub enum Direction {
    // Clockwise / Backward
    Cw,
    // Counter-clockwise / Forward
    Ccw,
}

pub trait Motor {
    fn run(&mut self, dir: Direction, speed: u8);

    fn coast(&mut self);

    fn stop(&mut self);
}