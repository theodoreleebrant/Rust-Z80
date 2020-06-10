// Defining possible flags in F
const SF: u8 = 0x80;    // 0b10000000
const ZF: u8 = 0x40;    // 0b01000000
const YF: u8 = 0x20;    // 0b00100000
const HF: u8 = 0x10;    // 0b00010000
const XF: u8 = 0x08;    // 0b00001000
const PF: u8 = 0x04;    // 0b00000100
const NF: u8 = 0x02;    // 0b00000010
const CF: u8 = 0x01;    // 0b00000001

// Implementing registers
pub struct Registers {
    pub mut A: u8,      // accumulator
    pub mut F: u8,      // flags

    // general registers
    pub mut B: u8,      
    pub mut C: u8,
    pub mut D: u8,
    pub mut E: u8,
    pub mut H: u8,
    pub mut L: u8,

    // Alternate register set
    pub mut A_: u8,
    pub mut F_: u8,

    pub mut B_: u8,
    pub mut C_: u8,
    pub mut D_: u8,
    pub mut E_: u8,
    pub mut H_: u8,
    pub mut L_: u8,

    // 16-bit combined registers
    pub mut BC: u16,
    pub mut DE: u16,
    pub mut HL: u16,        

    // 16-bit combined alternate registers
    pub mut BC_: u16,
    pub mut DE_: u16,
    pub mut HL_: u16,
    
    // Special registers
    pub mut I : u8,         // Interrupt vector**
    pub mut R : u8,         // Memory refresh
    pub mut IX: u16,        // Index register
    pub mut IY: u16,        // Index register
    pub mut PC: u16,        // Program counter
    pub mut SP: u16,        // Stack pointer
}


pub struct CPU {        // Move to own file later
    pub reg: Registers, // Already contains I, R, PC, SP, IX, IY 
    
    pub addr_bus: u16,  // 16-bit address bus
    pub data_bus: u8,   // 8-bit data bus

    pub ram: [u8; 65536],  // 64KB RAM
    pub clock_cycles: u32, // Timing matters 
}
