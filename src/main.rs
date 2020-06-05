use std::collections::HashMap;

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
struct Registers {
    A: u8,      // accumulator
    F: u8,      // flags

    // general registers
    B: u8,      
    C: u8,
    D: u8,
    E: u8,
    H: u8,
    L: u8,

    // Alternate register set
    A_: u8,
    F_: u8,

    B_: u8,
    C_: u8,
    D_: u8,
    E_: u8,
    H_: u8,
    L_: u8,

    // 16-bit registers
    AF: u16,        // Double up registers
    BC: u16,
    DE: u16,
    HL: u16,        

    // Special registers
    I : u8,         // Interrupt vector**
    R : u8,         // Memory refresh
    IX: u16,        // Index register
    IY: u16,        // Index register
    PC: u16,        // Program counter
    SP: u16,        // Stack pointer
}


struct CPU {
 





}




fn main() {
    println!("Hello, world!");
}
