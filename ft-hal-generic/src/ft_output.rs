/// The PWM intensity percentage for the high voltage level (Vcc).
const HIGH_INTENSITY: u8 = 100;
/// The PWM intensity percentage for the low voltage level (GND).
const LOW_INTENSITY: u8 = 0;

pub enum FtOutputCommand {
    /// Turn the output on with a specific PWM intensity (in percent).
    On(u8),
    /// Turn the output off by opening the circuit.
    Off,
    /// Connect the output to the high voltage level (Vcc). This is equivalent to `On(HIGH_INTENSITY)`.
    High,
    /// Connect the output to the low voltage level (GND).
    /// 
    /// Note: This is different from `Off`, where the circuit is open.
    Low,
}

pub trait FtOutput<E> {
    fn turn_on(&mut self, intensity: u8) -> Result<(), E>;

    fn turn_off(&mut self) -> Result<(), E>;

    fn set_output(&mut self, cmd: FtOutputCommand) -> Result<(), E> {
        match cmd {
            FtOutputCommand::On(intensity) => self.turn_on(intensity),
            FtOutputCommand::Off => self.turn_off(),
            FtOutputCommand::High => self.turn_on(HIGH_INTENSITY),
            FtOutputCommand::Low => self.turn_on(LOW_INTENSITY),
        }
    }
}
