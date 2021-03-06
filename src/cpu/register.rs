/// CPU registers, including registers specific to the ALU
///
/// The registers on the NES CPU are just like on the 6502. There is the accumulator, 2 indexes, a
/// program counter, the stack pointer, and the status register. Unlike many CPU families, members
/// do not have generic groups of registers like say, R0 through R7.
pub struct Registers {
    /// Accumulator register (A)
    pub acc: u8,

    /// Index register (X)
    ///
    /// It can be set to a value retrieved from memory and can be used to get or set the value of
    /// the stack pointer.
    pub x_idx: u8,

    /// Index register (Y)
    ///
    /// It can be set to a value retrieved from memory but cannot be used to get or set the value
    /// of the stack pointer.
    pub y_idx: u8,

    /// Program counter (PC)
    pub pc: u16,

    /// Stack pointer (SP)
    pub stack: u8,

    /// Status register (P)
    pub status: StatusFlags,
}

impl Default for Registers {
    fn default() -> Self {
        Registers {
            acc: 0,
            x_idx: 0,
            y_idx: 0,
            pc: 0xc00,
            stack: 0x24,
            status: StatusFlags::default(),
        }
    }
}

bitflags! {
    /// Status register
    ///
    ///  7 6 5 4 3 2 1 0
    ///  N V _ B D I Z C
    ///  | |   | | | | +--- Carry Flag
    ///  | |   | | | +----- Zero Flag
    ///  | |   | | +------- Interrupt Disable
    ///  | |   | +--------- Decimal Mode (unused)
    ///  | |   +----------- Break Command
    ///  | +--------------- Overflow Flag
    ///  +----------------- Negative Flag
    pub struct StatusFlags: u8 {
       const C_FLAG = 0b00000001;
       const Z_FLAG = 0b00000010;
       const I_FLAG = 0b00000100;
       const D_FLAG = 0b00001000; //unused, always on
       const B_FLAG = 0b00010000;
       const X_FLAG = 0b00100000; //unused, always on
       const V_FLAG = 0b01000000;
       const N_FLAG = 0b10000000;

       const NZ_FLAG = Self::N_FLAG.bits | Self::Z_FLAG.bits;
       const NZC_FLAG = Self::NZ_FLAG.bits | Self::C_FLAG.bits;
       const NVZC_FLAG = Self::NZC_FLAG.bits | Self::V_FLAG.bits;
       const NV_FLAG = Self::N_FLAG.bits | Self::V_FLAG.bits;
       const DX_FLAG = Self::D_FLAG.bits | Self::X_FLAG.bits;
    }
}

impl Default for StatusFlags {
    fn default() -> Self {
        Self::DX_FLAG
    }
}
