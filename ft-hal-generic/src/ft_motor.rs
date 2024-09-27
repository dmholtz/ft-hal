pub const MAX_SPEED: u8 = 100;

pub fn validate_speed(speed: u8) -> Result<(), ()> {
    if speed > MAX_SPEED {
        Err(())
    } else {
        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum FtMotorCommand {
    /// Drive forward with defined speed (in percent)
    Forward(u8),
    // Drive backward with defined speed (in percent)
    Backward(u8),
    /// Stop the motor by actively braking
    Stop,
    /// Stop the motor by coasting
    Coast,
}

pub trait FtMotor<E> {
    fn forward(&mut self, speed: u8) -> Result<(), E>;

    fn backward(&mut self, speed: u8) -> Result<(), E>;

    fn coast(&mut self) -> Result<(), E>;

    fn stop(&mut self) -> Result<(), E>;

    fn drive(&mut self, cmd: FtMotorCommand) -> Result<(), E> {
        match cmd {
            FtMotorCommand::Forward(speed) => self.forward(speed),
            FtMotorCommand::Backward(speed) => self.backward(speed),
            FtMotorCommand::Stop => self.stop(),
            FtMotorCommand::Coast => self.coast(),
        }
    }
}
