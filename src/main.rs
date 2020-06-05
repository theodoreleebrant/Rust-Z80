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
    pub A: u8,      // accumulator
    pub F: u8,      // flags

    // general registers
    pub B: u8,      
    pub C: u8,
    pub D: u8,
    pub E: u8,
    pub H: u8,
    pub L: u8,

    // Alternate register set
    pub A_: u8,
    pub F_: u8,

    pub B_: u8,
    pub C_: u8,
    pub D_: u8,
    pub E_: u8,
    pub H_: u8,
    pub L_: u8,

    // 16-bit combined registers
    pub BC: u16,
    pub DE: u16,
    pub HL: u16,        

    // 16-bit combined alternate registers
    pub BC_: u16,
    pub DE_: u16,
    pub HL_: u16,
    
    // Special registers
    pub I : u8,         // Interrupt vector**
    pub R : u8,         // Memory refresh
    pub IX: u16,        // Index register
    pub IY: u16,        // Index register
    pub PC: u16,        // Program counter
    pub SP: u16,        // Stack pointer
}


pub struct CPU {        // Move to own file later
    pub reg: Registers, 
    
    pub bus: Bus,       // Implement data bus separately
    pub address: u16,
    pub clock_cycles: u32, 
}




fn main() {
    println!("Hello, world!");
}
