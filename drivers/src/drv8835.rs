use core::marker::PhantomData;

use embedded_hal::digital::OutputPin;

pub use ft_hal_generic::motor;
pub use ft_hal_generic::motor::Motor;

// type-state pattern: encode the mode selector in the type system
pub trait ModeSelectorMarker {}
pub enum HardWired {}
pub enum ModePin {}
impl ModeSelectorMarker for HardWired {}
impl ModeSelectorMarker for ModePin {}

// type-state pattern: encode the mode (state) of the driver in the type system
pub trait ModeMarker {}
pub enum InIn {}
pub enum PhaseEnable {}
impl ModeMarker for InIn {}
impl ModeMarker for PhaseEnable {}

pub struct Drv8835Pins<T: OutputPin, S: ModeSelectorMarker> {
    in1a_pin: T,
    in2a_pin: T,
    in1b_pin: T,
    in2b_pin: T,

    mode_selector: PhantomData<S>,
    mode_pin: Option<T>,
}

impl<T: OutputPin> Drv8835Pins<T, ModePin> {
    pub fn new_with_mode_pin(in1a_pin: T, in2a_pin: T, in1b_pin: T, in2b_pin: T, mode_pin: T) -> Drv8835Pins<T, ModePin> {
        Drv8835Pins {
            in1a_pin: in1a_pin,
            in2a_pin: in2a_pin,
            in1b_pin: in1b_pin,
            in2b_pin: in2b_pin,

            mode_selector: PhantomData::<ModePin>,
            mode_pin: Some(mode_pin),
        }
    }
}

impl<T: OutputPin> Drv8835Pins<T, HardWired> {
    pub fn new_without_mode_pin(in1a_pin: T, in2a_pin: T, in1b_pin: T, in2b_pin: T) -> Drv8835Pins<T, HardWired> {
        Drv8835Pins {
            in1a_pin: in1a_pin,
            in2a_pin: in2a_pin,
            in1b_pin: in1b_pin,
            in2b_pin: in2b_pin,

            mode_selector: PhantomData::<HardWired>,
            mode_pin: None,
        }
    }
}

pub struct Drv8835<T: OutputPin, M: ModeMarker, S: ModeSelectorMarker> {
    pins: Drv8835Pins<T, S>,
    mode_marker: PhantomData<M>,
    mode_selector: PhantomData<S>,
}

impl<T: OutputPin, M: ModeMarker, S: ModeSelectorMarker> From<Drv8835Pins<T, S>> for Drv8835<T, M, S> {
    fn from(pins: Drv8835Pins<T, S>) -> Self {
        Drv8835 {
            pins: pins,
            mode_marker: PhantomData::<M>,
            mode_selector: PhantomData::<S>,
        }
    }
}

impl <T: OutputPin, M: ModeMarker, S: ModeSelectorMarker> Drv8835<T, M, S> {
    pub fn into_driver_pair(self) -> (Drv8835Driver<T, M>, Drv8835Driver<T, M>) {
        (
            Drv8835Driver::new(self.pins.in1a_pin, self.pins.in2a_pin, PhantomData::<M>),
            Drv8835Driver::new(self.pins.in1b_pin, self.pins.in2b_pin, PhantomData::<M>),
        )
    }
}

impl<T: OutputPin, M: ModeMarker> Drv8835<T, M, ModePin> {
    pub fn into_in_in_mode(mut self) -> Drv8835<T, InIn, ModePin> {
        if let Some(ref mut pin) = self.pins.mode_pin {
            pin.set_low();
        }
        Drv8835 {
            pins: self.pins,
            mode_marker: PhantomData::<InIn>,
            mode_selector: PhantomData::<ModePin>,
        }
    }

    pub fn into_phase_enable_mode(mut self) -> Drv8835<T, PhaseEnable, ModePin> {
        if let Some(ref mut pin) = self.pins.mode_pin {
            pin.set_high();
        }
        Drv8835 {
            pins: self.pins,
            mode_marker: PhantomData::<PhaseEnable>,
            mode_selector: PhantomData::<ModePin>,
        }
    }
}

pub struct Drv8835Driver<T: OutputPin, M: ModeMarker> {
    in1_pin: T,
    in2_pin: T,

    mode: PhantomData<M>,
}

impl<T: OutputPin, M: ModeMarker> Drv8835Driver<T, M> {
    pub fn new(in1_pin: T, in2_pin: T, mode: PhantomData<M>) -> Self {
        Drv8835Driver {
            in1_pin: in1_pin,
            in2_pin: in2_pin,
            mode: mode,
        }
    }
}

impl<T: OutputPin> Drv8835Driver<T, InIn> {
    pub fn into_motor(self) -> impl Motor {
        self
    }

    pub fn into_output_pair(self) -> (u8, u8) {
        (0,0)
    }
}

impl<T: OutputPin> Drv8835Driver<T, PhaseEnable> {
    fn get_enable_pin(&mut self) -> &mut T {
        &mut self.in2_pin
    }

    fn get_phase_pin(&mut self) -> &mut T {
        &mut self.in1_pin
    }

    pub fn into_motor(self) -> impl Motor {
        self
    }
}

impl<T: OutputPin> Motor for Drv8835Driver<T, InIn> {
    fn run(&mut self, dir: motor::Direction, speed: u8) {
        match dir {
            motor::Direction::Cw => {
                self.in1_pin.set_low();
                self.in2_pin.set_high(); // TODO: with PWM
            }
            motor::Direction::Ccw => {
                self.in1_pin.set_high(); // TODO: with PWM
                self.in2_pin.set_low();    
            }
        }
    }

    fn coast(&mut self) {
        self.in1_pin.set_low();
        self.in2_pin.set_low();
    }

    fn stop(&mut self) {
        self.in1_pin.set_high();
        self.in2_pin.set_high();
    }
}

impl<T: OutputPin> Motor for Drv8835Driver<T, PhaseEnable> {
    fn run(&mut self, dir: motor::Direction, speed: u8) {
        let enable_pin = self.get_enable_pin();
        enable_pin.set_high(); // TODO: with PWM

        let phase_pin = self.get_phase_pin();
        match dir {
            motor::Direction::Cw => {
                phase_pin.set_high();
            }
            motor::Direction::Ccw => {
                phase_pin.set_low();
            }
        }
    }

    fn coast(&mut self) {
        // not implemented default to stop
        self.stop();
    }

    fn stop(&mut self) {
        let enable_pin = self.get_enable_pin();
        enable_pin.set_low();

        // phase pin can be left high or low: we make it deterministic
        let phase_pin = self.get_phase_pin();
        phase_pin.set_low();
    }
}
