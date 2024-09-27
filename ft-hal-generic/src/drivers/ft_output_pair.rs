use core::marker::PhantomData;

use crate::{
    ft_motor::{validate_speed, FtMotor},
    ft_output::{FtOutput, FtOutputCommand},
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MotorError<OUT1Error, OUT2Error> {
    InvalidSpeed,
    Output1Error(OUT1Error),
    Output2Error(OUT2Error),
}

/// A pair of FtOutput instances that can be used as a motor.
pub struct FtOutputPair<O1, O2, E1, E2>
where
    O1: FtOutput<E1>,
    O2: FtOutput<E2>,
{
    output1: O1,
    output2: O2,

    _err1: PhantomData<E1>,
    _err2: PhantomData<E2>,
}

impl<O1, O2, E1, E2> FtOutputPair<O1, O2, E1, E2>
where
    O1: FtOutput<E1>,
    O2: FtOutput<E2>,
{
    pub fn new(output1: O1, output2: O2) -> Self {
        FtOutputPair {
            output1,
            output2,
            _err1: PhantomData,
            _err2: PhantomData,
        }
    }

    pub fn into_motor(self) -> impl FtMotor<MotorError<E1, E2>> {
        self
    }
}

impl<O1, O2, E1, E2> FtMotor<MotorError<E1, E2>> for FtOutputPair<O1, O2, E1, E2>
where
    O1: FtOutput<E1>,
    O2: FtOutput<E2>,
{
    fn forward(&mut self, speed: u8) -> Result<(), MotorError<E1, E2>> {
        validate_speed(speed).map_err(|_| MotorError::InvalidSpeed)?;
        self.output1
            .turn_on(speed)
            .map_err(MotorError::Output1Error)?;
        self.output2
            .set_output(FtOutputCommand::Low)
            .map_err(MotorError::Output2Error)?;
        Ok(())
    }

    fn backward(&mut self, speed: u8) -> Result<(), MotorError<E1, E2>> {
        validate_speed(speed).map_err(|_| MotorError::InvalidSpeed)?;
        self.output1
            .set_output(FtOutputCommand::Low)
            .map_err(MotorError::Output1Error)?;
        self.output2
            .turn_on(speed)
            .map_err(MotorError::Output2Error)?;
        Ok(())
    }

    fn coast(&mut self) -> Result<(), MotorError<E1, E2>> {
        self.output1
            .turn_off()
            .map_err(MotorError::Output1Error)?;
        self.output2
            .turn_off()
            .map_err(MotorError::Output2Error)?;
        Ok(())
    }

    fn stop(&mut self) -> Result<(), MotorError<E1, E2>> {
        self.output1
            .set_output(FtOutputCommand::Low)
            .map_err(MotorError::Output1Error)?;
        self.output2
            .set_output(FtOutputCommand::Low)
            .map_err(MotorError::Output2Error)?;
        Ok(())
    }
}
