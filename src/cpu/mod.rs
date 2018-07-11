use self::register::Registers;

use clock::Processor;
use memory::Memory;

mod instruction;
mod register;

pub struct Core {
    pub reg: Registers,
}

impl Processor for Core {
    fn cycle(&mut self, memory: &mut Memory) {
        let opcode = memory.fetch(self.reg.pc);
        let _cycles = self.execute(opcode, memory);

        // TODO(joshleeb): Timing (use returned cycles).
    }
}

impl Core {
    pub fn new(reg: Registers) -> Self {
        Core { reg }
    }

    /// Absolute address.
    pub fn abs_addr(&mut self, memory: &mut Memory) -> u16 {
        let lo = memory.fetch(self.reg.pc) as u16;
        let hi = memory.fetch(self.reg.pc + 1) as u16;
        self.reg.pc += 2;

        lo | hi << 8
    }

    /// Indirect address.
    ///
    /// The 6502 processor has a bug in which only the high byte is incremented instead of the
    /// whole 16-bit address when computing the indirect address. See
    /// http://www.6502.org/tutorials/6502opcodes.html#JMP for details.
    pub fn indr_addr(&mut self, memory: &mut Memory) -> u16 {
        let lo_addr = memory.fetch(self.reg.pc) as u16;
        let hi_addr = memory.fetch(self.reg.pc + 1) as u16;
        self.reg.pc += 2;

        let lo_adjusted = lo_addr + 1 | hi_addr << 8;
        let hi_adjusted = lo_addr | hi_addr << 8;

        let lo = memory.fetch(lo_adjusted) as u16;
        let hi = memory.fetch(hi_adjusted) as u16;
        lo | hi << 8
    }

    /// Execute the opcode and return the number of cycles.
    pub fn execute(&mut self, opcode: u8, memory: &mut Memory) -> usize {
        self.reg.pc += 1;

        match opcode {
            0x4c => self.jump_abs(memory),
            0x6c => self.jump_indr(memory),
            _ => unimplemented!(),
        }
    }
}
