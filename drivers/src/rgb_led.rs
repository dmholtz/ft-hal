use embedded_hal::digital::OutputPin;

#[derive(Clone, Copy, PartialEq)]
pub enum Color {
    // Off state
    Off,

    // Primary colors
    Red,
    Green,
    Blue,

    // Mixtures
    Yellow,
    Cyan,
    Magenta,
    White,
}

pub struct RgbLed<T: OutputPin> {
    r_pin: T,
    g_pin: T,
    b_pin: T,
}

impl<T: OutputPin> RgbLed<T> {
    pub fn new(r_pin: T, g_pin: T, b_pin: T) -> Self {
        RgbLed {
            r_pin: r_pin,
            g_pin: g_pin,
            b_pin: b_pin,
        }
    }

    pub fn off(&mut self) {
        self.r_pin.set_low();
        self.g_pin.set_low();
        self.b_pin.set_low();
    }

    pub fn show_color(&mut self, color: Color) {
        match color {
            Color::Off => self.off(),
            Color::Red => {
                self.r_pin.set_high();
                self.g_pin.set_low();
                self.b_pin.set_low();
            }
            Color::Green => {
                self.r_pin.set_low();
                self.g_pin.set_high();
                self.b_pin.set_low();
            }
            Color::Blue => {
                self.r_pin.set_low();
                self.g_pin.set_low();
                self.b_pin.set_high();
            }
            Color::Yellow => {
                self.r_pin.set_high();
                self.g_pin.set_high();
                self.b_pin.set_low();
            }
            Color::Cyan => {
                self.r_pin.set_low();
                self.g_pin.set_high();
                self.b_pin.set_high();
            }
            Color::Magenta => {
                self.r_pin.set_high();
                self.g_pin.set_low();
                self.b_pin.set_high();
            }
            Color::White => {
                self.r_pin.set_high();
                self.g_pin.set_high();
                self.b_pin.set_high();
            }
        }
    }
}