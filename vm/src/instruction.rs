// opcodes have 3 character mnemonics.
// when implementing an assembly language for the processor, the following ones should be used.
// the table also describes valid source and destination operands as such:
// [source] -> [destination].
// .   : no operand.
// gpr : general purpose register.
// imm : instruction immediate value.
pub enum Opcode {
    Inv = 0, // invalid             : [.] -> [.].
    Nop = 1, // no operation        : [.] -> [.].
    Hlt = 2, // halt execution      : [.] -> [.].
    Set = 3, // set gp register     : [gpr/imm] -> [gpr].
    Srd = 4, // set ram device      : [gpr/imm] -> [.].
    Lod = 5, // load from ram       : [gpr/imm] -> [gpr].
    Str = 6, // store to ram        : [gpr/imm] -> [gpr/imm].
    Add = 7, // add to gp register  : [gpr/imm] -> [gpr].
}

impl From<u8> for Opcode {
    fn from(opcode: u8) -> Self {
        match opcode {
            1 => Opcode::Nop,
            2 => Opcode::Hlt,
            3 => Opcode::Set,
            4 => Opcode::Srd,
            5 => Opcode::Lod,
            6 => Opcode::Str,
            7 => Opcode::Add,
            _ => Opcode::Inv,
        }
    }
}

pub enum Operand {
    Invalid = 0,
    GpRegister0 = 1,
    GpRegister1 = 2,
    GpRegister2 = 3,
    GpRegister3 = 4,
    InstImm = 5,
}

impl From<u8> for Operand {
    fn from(operand: u8) -> Self {
        match operand {
            1 => Operand::GpRegister0,
            2 => Operand::GpRegister1,
            3 => Operand::GpRegister2,
            4 => Operand::GpRegister3,
            5 => Operand::InstImm,
            _ => Operand::Invalid,
        }
    }
}

pub struct Instruction {
    opcode: u8,

    // instructions can have up to two operands, which are stored in this byte.
    // bits 0..4 are the source operand.
    // bits 4..8 are the destination operand.
    operands: u8,
    
    imm: u16,
}

impl Instruction {
    pub fn new(opcode: Opcode, src: Operand, dst: Operand) -> Self {
        Self {
            opcode: opcode as u8,
            operands: src as u8 | (dst as u8) << 4,
            imm: 0,
        }
    }

    pub fn with_imm(opcode: Opcode, src: Operand, dst: Operand, imm: u16) -> Self {
        let mut inst = Self::new(opcode, src, dst);
        inst.imm = imm;
        inst
    }

    pub fn opcode(&self) -> Opcode {
        Opcode::from(self.opcode)
    }

    pub fn src(&self) -> Operand {
        Operand::from(self.operands & 0xf)
    }

    pub fn dst(&self) -> Operand {
        Operand::from((self.operands & 0xf0) >> 4)
    }

    pub fn imm(&self) -> u16 {
        self.imm
    }
}
