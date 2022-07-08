use crate::device::Device;
use crate::instruction::{Instruction, Operand, Opcode};
use std::error::Error;
use std::fmt;

pub enum CpuFlags {
    // whenever the cpu executes a nop instruction, the next instruction is skipped.
    // set when the nop is about to happen.
    // clear when instructions are executed as expected (not being skipped).
    Nop = 1,
}

pub struct Cpu {
    gp_regs: [u16; 4],
    flags_reg: u16,
    inst_ptr: u16,
    active_ram_dev: u16, // index of the active ram device in `devs`.
    devs: Vec<Box<dyn Device>>,
}

impl fmt::Display for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "gpr = {:?}, fr = {}, ip = {}, ard = {}",
            self.gp_regs,
            self.flags_reg,
            self.inst_ptr,
            self.active_ram_dev,
        )
    }
}

#[derive(Debug)]
pub enum CpuError {
    TooManyDevices,
    DeviceIdExists,
    DeviceDoesNotExist,
    ProgramError,
}

impl fmt::Display for CpuError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for CpuError {}

impl Cpu {
    pub fn new() -> Self {
        Self {
            gp_regs: [0; 4],
            flags_reg: 0,
            inst_ptr: 0,
            active_ram_dev: 0,
            devs: Vec::new(),
        }
    }

    // due to the 16 bit id system of devices, you cannot connect more than 65536 devices.
    // you also cannot add a device with the id of an existing one.
    pub fn connect_device(&mut self, dev: Box<dyn Device>) -> Result<(), CpuError> {
        if self.devs.len() >= 65536 {
            return Err(CpuError::TooManyDevices);
        }
        if self.devs.iter().filter(|d| d.id() == dev.id()).collect::<Vec<_>>().len() > 0 {
            Err(CpuError::DeviceIdExists)
        } else {
            self.devs.push(dev);
            Ok(())
        }
    }

    pub fn set_ram_dev(&mut self, id: u16) -> Result<(), CpuError> {
        let devs_with_id = self.devs
            .iter()
            .enumerate()
            .filter(|(_, d)| d.id() == id)
            .map(|(i, _)| i)
            .collect::<Vec<_>>();
        if devs_with_id.len() == 0 {
            Err(CpuError::DeviceDoesNotExist)
        } else {
            self.active_ram_dev = devs_with_id[0] as u16;
            Ok(())
        }
    }

    fn src_operand(&self, inst: &Instruction) -> u16 {
        match inst.src() {
            Operand::Invalid => 0,
            Operand::GpRegister0 => self.gp_regs[0],
            Operand::GpRegister1 => self.gp_regs[1],
            Operand::GpRegister2 => self.gp_regs[2],
            Operand::GpRegister3 => self.gp_regs[3],
            Operand::InstImm => inst.imm(),
        }
    }

    fn dst_operand(&mut self, inst: &Instruction) -> Option<&mut u16> {
        match inst.dst() {
            Operand::GpRegister0 => Some(&mut self.gp_regs[0]),
            Operand::GpRegister1 => Some(&mut self.gp_regs[1]),
            Operand::GpRegister2 => Some(&mut self.gp_regs[2]),
            Operand::GpRegister3 => Some(&mut self.gp_regs[3]),
            _ => None,
        }
    }

    fn active_ram_dev(&mut self) -> &mut Box<dyn Device> {
        &mut self.devs[self.active_ram_dev as usize]
    }

    pub fn exec_instruction(&mut self, inst: Instruction) -> Result<(), Box<dyn Error>> {
        if self.flags_reg & CpuFlags::Nop as u16 > 0 {
            self.flags_reg &= !(CpuFlags::Nop as u16);
            return Ok(());
        }

        match inst.opcode() {
            Opcode::Inv => return Err(Box::new(CpuError::ProgramError)),
            Opcode::Nop => self.flags_reg |= CpuFlags::Nop as u16,
            Opcode::Hlt => (),
            Opcode::Set => *self.dst_operand(&inst).unwrap() = self.src_operand(&inst),
            Opcode::Srd => self.set_ram_dev(self.src_operand(&inst))?,
            Opcode::Lod => {
                let src_op = self.src_operand(&inst);
                *self.dst_operand(&inst).unwrap() = self.active_ram_dev().act_primary(src_op)?;
            }
            Opcode::Str => {
                let src_op = self.src_operand(&inst);
                let dst_op = *self.dst_operand(&inst).unwrap();
                self.active_ram_dev().act_secondary(src_op, dst_op)?;
            }
            Opcode::Add => {
                let src_op = self.src_operand(&inst);
                *self.dst_operand(&inst).unwrap() += src_op;
            }
        }
        
        Ok(())
    }
}
