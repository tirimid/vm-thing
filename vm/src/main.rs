mod cpu;
mod device;
mod ram;
mod instruction;

use crate::cpu::Cpu;
use crate::ram::Ram;
use crate::instruction::{Opcode, Operand, Instruction};

fn main() {
    let mut cpu = Cpu::new();
    cpu.connect_device(Box::new(Ram::new(7))).unwrap();
    cpu.set_ram_dev(7).unwrap();

    let inst = Instruction::with_imm(Opcode::Set, Operand::InstImm, Operand::GpRegister0, 100);
    cpu.exec_instruction(inst).unwrap();
    println!("{}", cpu);
}
