use crate::device::{DeviceError, Device};

const RAM_SIZE: usize = 256 * 256; // this allows for a 16 bit address space.

pub struct Ram {
    id: u16,
    data: [u8; RAM_SIZE],
}

impl Ram {
    pub fn new(id: u16) -> Self {
        Self { id, data: [0; RAM_SIZE] }
    }
}

impl Device for Ram {
    fn id(&self) -> u16 {
        self.id
    }

    // dereferences the word in memory at address `input`.
    // memory addresses are rounded down to the nearest 16 bit address.
    fn act_primary(&mut self, input: u16) -> Result<u16, DeviceError> {
        let input = (input - input % 2) as usize;
        Ok((self.data[input] as u16) << 8 | self.data[input + 1] as u16)
    }

    // writes word `input_0` in memory at address `input_1`.
    // memory addresses are rounded down to the nearest 16 bit address.
    // returns `NoOutput` error under normal circumstances.
    fn act_secondary(&mut self, input_0: u16, input_1: u16) -> Result<u16, DeviceError> {
        let input_1 = (input_1 - input_1 % 2) as usize;
        self.data[input_1] = (input_0 >> 8) as u8;
        self.data[input_1 + 1] = input_0 as u8;
        Err(DeviceError::NoOutput)
    }
}
