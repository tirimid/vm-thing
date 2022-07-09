use crate::device::Device;
use std::error::Error;
use std::fmt;

pub struct Bus {
    devs: Vec<Box<dyn Device>>,
}

#[derive(Debug)]
pub enum BusError {
    DeviceIdExists,
}

impl Error for BusError {}
impl fmt::Display for BusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Bus {
    pub fn new() -> Self {
        Self { devs: Vec::new() }
    }

    // returns indices of all devices with a certain id.
    // returned `Vec` should only ever have zero/one element(s) under normal circumstances.
    fn devs_with_id(&self, id: u32) -> Vec<u32> {
        self.devs
            .iter()
            .enumerate()
            .filter(|(_, dev)| id == dev.header().id())
            .map(|(i, _)| i as u32)
            .collect()
    }

    pub fn connect_dev(&mut self, dev: Box<dyn Device>) -> Result<(), BusError> {
        if self.devs_with_id(dev.header().id()).len() > 0 {
            Err(BusError::DeviceIdExists)
        } else {
            self.devs.push(dev);
            Ok(())
        }
    }
}
