pub enum Direction {
    Cw,
    Ccw,
}

pub trait Motor {
    fn run(&self, dir: Direction, speed: u8);

    fn coast(&self);

    fn stop(&self);
}