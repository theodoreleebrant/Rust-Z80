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

// Implementing enum for program counter
enum ProgramCounter {
    Next(u8),                   // Z80 instructions vary in length. provide number of bytes instruction takes up.
    Jump(u16),
}

pub struct CPU {        // Move to own file later
    pub reg: Registers, // Already contains I, R, PC, SP, IX, IY 
    
    pub addr_bus: u16,  // 16-bit address bus
    pub data_bus: u8,   // 8-bit data bus

    pub ram: [u8; 65536],  // 64KB RAM
    pub clock: u32, // Timing matters
    // Do we need to implement control lines??
}

impl CPU {
    // Functions for initalization, load program, tick, run opcode.
    // For now, dk what's the state of Z80 when first initialized.
    
    /// load_to takes in a register ID reg_id and content.
    /// Loads content into specified register.
    pub fn load_to(&self, reg_id: u8, content: u8) -> bool {
        match reg_id {
            0 => self.reg.B = content,
            1 => self.reg.C = content,
            2 => self.reg.D = content,
            3 => self.reg.E = content,
            4 => self.reg.H = content,
            5 => self.reg.L = content,
            7 => self.reg.A = content,
            .. => return false,
        }

        true
    }

    /// load_from takes in a register ID reg_id.
    /// Outputs content of register as an Option<u8>, returns None if reg_id is not valid.
    pub fn load_from(&self, reg_id: u8) -> Option<u8> {
        Option<u8> result;

        match reg_id {
            0 => result = Some(self.reg.B),
            1 => result = Some(self.reg.C),
            2 => result = Some(self.reg.D),
            3 => result = Some(self.reg.E),
            4 => result = Some(self.reg.H),
            5 => result = Some(self.reg.L),
            7 => result = Some(self.reg.A),
            .. => result = None,
        }

        result
    }

    // OPCODES GOES HERE
    // Notation:
    // ````` Value of Register `````
    // r: Register
    // (HL): content of memory location stored in register HL
    // (IX+d) or (IY+d): content of memory location IX with offset d
    // ````` Data types `````
    // n: one-byte unsigned int
    // nn: two-byte unsigned int
    // d: one-byte signed int
    // b: one-bit expression in range (0 to 7)??
    // e: one-byte signed int for relative jump offset
    // ````` Register `````
    // cc: status of Flag Register as any flag
    // qq: BC, DE, HL or AF
    // ss: BC, DE, HL or SP
    // pp: BC, DE, IX or SP
    // rr: BC, DE, IY or SP
    // ````` General `````
    // s: Any of r, n, (HL), (IX+d) or (IY+d)
    // m: Any of r, (HL), (IX+d) or (IY+d) (no n)

    // 8-bit Load group
    //
    
    /// 01rxry: given 2 registers rx and ry, load value of ry into rx,
    /// 1-byte instruction
    pub fn op_01rxry(rx: u8, ry: u8) -> ProgramCounter {
       match self.load_from(ry) {
           None => (), // FIXME: How do i handle error here
           Some(value) => self.load_to(rx, value),
       }

       ProgramCounter::Next(1) // Increment the program counter
    }
           
    /// 00rn: given register r and immediate u8 value n. load n into r
    /// 2-byte instruction
    pub fn op_00rn(r: u8) -> ProgramCounter {
        let n = self.ram[self.reg.PC + 1]; // value of intermediate is at next memory slot
        self.load_to(r, n);

        ProgramCounter::Next(2)
    }

    /// 01rHL: given register r, load value pointed to by value in register HL into r.
    /// HL is implied and is not included in instruction
    /// 1-byte instruction
    pub fn op_01rHL(r: u8) -> ProgramCounter {
        let hl = self.reg.HL;
        self.load_to(r, self.ram[hl]);
        
        ProgramCounter::Next(1)
    }

    /// DDrd: given register r and offset d, load contents of IX + offset d to register r.
    /// 3-byte instruction
    pub fn op_DDrd(r: u8, d: i8) -> ProgramCounter {
        let res = self.reg.IX + d;
        self.load_to(r, res);
        
        ProgramCounter::Next(3)
    }
    
    /// FDrd: given register r and offset d, load contents of IX + offset d to register r.
    /// 3-byte instruction
    pub fn op_FDrd(r: u8, d: i8) -> ProgramCounter {
        let res = self.reg.IY + d;
        self.load_to(r, res);
        
        ProgramCounter::Next(3)
    }

    /// 36n: Given immediate n. n is loaded into the memory address specified by contents of HL
    /// register.
    /// 2-byte instruction.

    pub fn op_36n(n: u8) -> ProgramCounter {
        self.ram[self.reg.HL] = n;

        ProgramCounter::Next(2)
    }

    /// DD36dn: Given immediate n and offset d. n is loaded into the memory address specified by 









}
