use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum DeviceError {
    NoOutput,
    Invalid,
}

impl fmt::Display for DeviceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for DeviceError {}

pub trait Device {
    // all devices must have an id in order to be retrieved and used by the cpu.
    fn id(&self) -> u16;
    
    // every device does something when called on by the cpu.
    // the returned value should be the result of this action.
    // some devices have multiple actions they can do.
    fn act_primary(&mut self, input: u16) -> Result<u16, DeviceError>;
    fn act_secondary(&mut self, input_0: u16, input_1: u16) -> Result<u16, DeviceError>;
}
