use crate::device::{Device, DeviceError, DeviceHeader, DeviceType};

pub struct Ram {
    hdr: DeviceHeader,
    mem: Box<[u32]>,
}

pub enum RamWorkAction {
    Invalid = 0,
    Read = 1,
    Write = 2,
}

impl From<u32> for RamWorkAction {
    fn from(u: u32) -> Self {
        match u {
            1 => Self::Read,
            2 => Self::Write,
            _ => Self::Invalid,
        }
    }
}

impl Ram {
    // `mem_size` defines size in words (`u32`s).
    // a `mem_size` of 32 would create a memory with the size of 128 bytes.
    pub fn new(id: u32, mem_size: u32) -> Self {
        Self {
            hdr: DeviceHeader::new(DeviceType::Ram, id, &[mem_size]),
            mem: vec![0; mem_size as usize].into_boxed_slice(),
        }
    }

    fn access_mem(&mut self, ptr: u32) -> Result<&mut u32, DeviceError> {
        if ptr as usize >= self.mem.len() {
            Err(DeviceError::InvalidInput)
        } else {
            Ok(&mut self.mem[ptr as usize])
        }
    }
}

impl Device for Ram {
    fn header(&self) -> &DeviceHeader {
        &self.hdr
    }

    // args: [1 (read), location] -> [word].
    //       [2 (write), location, word] -> [0].
    fn work(&mut self, args: &[u32]) -> Result<Box<[u32]>, DeviceError> {
        match RamWorkAction::from(args[0]) {
            RamWorkAction::Invalid => Err(DeviceError::InvalidInput),
            RamWorkAction::Read => {
                if args.len() != 2 {
                    Err(DeviceError::WrongArgumentCount)
                } else {
                    let word = *self.access_mem(args[1])?;
                    Ok(Box::new([word]))
                }
            }
            RamWorkAction::Write => {
                if args.len() != 3 {
                    Err(DeviceError::WrongArgumentCount)
                } else {
                    *self.access_mem(args[1])? = *self.access_mem(args[2])?;
                    Ok(Box::new([0]))
                }
            }
        }
    }
}
