use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum DeviceError {
    InvalidInput,
    WrongArgumentCount,
}

impl Error for DeviceError {}
impl fmt::Display for DeviceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Copy, Clone)]
pub enum DeviceType {
    Ram,
    Cpu,
    Disk,
    Display,
}

pub struct DeviceHeader {
    dev_type: DeviceType,
    id: u32,

    // some devices need to store some data which can be retrieved.
    // for example, `Ram` needs to store how much memory is available to it.
    info: Box<[u32]>,
}

impl DeviceHeader {
    pub fn new(dev_type: DeviceType, id: u32, info: &[u32]) -> Self {
        Self {
            dev_type,
            id,
            info: info.to_vec().into_boxed_slice(),
        }
    }
    
    pub fn dev_type(&self) -> DeviceType {
        self.dev_type
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn info(&self) -> &[u32] {
        &self.info
    }
}

pub trait Device {
    fn header(&self) -> &DeviceHeader;
    fn work(&mut self, args: &[u32]) -> Result<Box<[u32]>, DeviceError>;
}
