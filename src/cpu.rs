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
    pub mut BC: usize,      // (usize to handle indexing)
    pub mut DE: usize,      // (usize to handle indexing)
    pub mut HL: usize,      // (usize to handle indexing)

    // 16-bit combined alternate registers
    pub mut BC_: usize,     // (usize to handle indexing)
    pub mut DE_: usize,     // (usize to handle indexing)
    pub mut HL_: usize,     // (usize to handle indexing)
    
    // Special registers
    pub mut I : u8,         // Interrupt vector**
    pub mut R : u8,         // Memory refresh
    pub mut IX: usize,      // Index register (usize to handle indexing)
    pub mut IY: usize,      // Index register (usize to handle indexing)
    pub mut PC: usize,      // Program counter (usize to handle indexing)
    pub mut SP: usize,      // Stack pointer (usize to handle indexing)
}

// Implementing enum for program counter
enum ProgramCounter {
    Next(u8),                   // Z80 instructions vary in length. provide number of bytes instruction takes up.
    Jump(u16),
}

pub struct CPU {        // Move to own file later
    pub reg: Registers, // Already contains I, R, PC, SP, IX, IY 
    
    pub mut addr_bus: u16,  // 16-bit address bus
    pub mut data_bus: u8,   // 8-bit data bus

    pub mut ram: [u8; 65536],  // 64KB RAM
    pub mut clock: u32, // Timing matters
    
    // Control signals. Supposedly just bits but Rust doesn't treat boolean as 0 and 1. Check out
    // Z80 manual page 31.
    pub mut halt: u8,
    pub mut iff1: u8,       // disable interrupts from being accepted
    pub mut iff2: u8,       // temp. storage for iff1
    pub mut ei:   u8,       // Enable Interrupt signal
    pub mut im:   u8,       // Interrupt Mode 0,1,2
    pub mut nmi:  u8,       // non-maskable interrupt
    pub mut int:  u8,       // maskable interrupt
}

impl CPU {
    // Functions for initalization, load program, tick, run opcode.
    // For now, dk what's the state of Z80 when first initialized.
   
    /* Reusable code */

    /// load_to takes in a 1-byte register ID reg_id and content.
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

    /// load_from takes in a 1-byte register ID reg_id.
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

        if result == Some(None) {
            return None;
        }

        result
    }

    /// load_nn_to takes in 2-byte register ID reg_id and content to write. Outputs an optional
    /// boolean value to indicate whether reg_id is valid
    pub fn load_nn_to(&self, reg_id: u8, content: u16) -> bool {
        match reg_id {
            0 => self.reg.BC = content as usize,
            1 => self.reg.DE = content as usize,
            2 => self.reg.HL = content as usize,
            3 => self.reg.SP = content as usize,
            .. => return false,
        }

        true
    }

    /// load_nn_from takes in 2-byte register ID reg_id, and retrieve its content.
    /// Outputs an Option<u16>, or None if reg_id is invalid.
    pub fn load_nn_from(&self, reg_id: u8) -> Option<u16> {
        Option<u16> result;

        match reg_id {
            0 => result = Some(self.reg.BC as u16),
            1 => result = Some(self.reg.DE as u16),
            2 => result = Some(self.reg.HL as u16),
            3 => result = Some(self.reg.SP as u16),
            .. => result = None,
        }

        if result == Some(None) {
            return None;
        }

        result
    }

    // NOTATION
    //
    // ````` Value of Register `````
    // r / x / y: Register
    // (HL): content of memory location stored in register HL
    // (IX+d) or (IY+d): content of memory location IX with offset d
    // 
    // ````` Data types `````
    // n: one-byte unsigned int
    // nn: two-byte unsigned int. 
    // *First n operand after the opcode is lower-order byte
    // d: one-byte signed int
    // b: one-bit expression in range (0 to 7)??
    // e: one-byte signed int for relative jump offset
    // 
    // ````` Register `````
    // cc: status of Flag Register as any flag
    // qq: BC, DE, HL or AF
    // ss: BC, DE, HL or SP
    // pp: BC, DE, IX or SP
    // rr: BC, DE, IY or SP
    // 
    // ````` General `````
    // s: Any of r, n, (HL), (IX+d) or (IY+d)
    // m: Any of r, (HL), (IX+d) or (IY+d) (no n)

    /* 8-bit Load group */
        
    /// ld_x_y: given 2 registers rx and ry, load value of ry into rx,
    /// 1-byte instruction
    pub fn ld_x_y(rx: u8, ry: u8) -> ProgramCounter {
       match self.load_from(ry) {
           None => (), // FIXME: How do i handle error here
           Some(value) => self.load_to(rx, value),
       }

       ProgramCounter::Next(1) // Increment the program counter
    }
           
    /// ld_r_n: given register r and immediate u8 value n. load n into r
    /// 2-byte instruction
    pub fn ld_r_n(r: u8, n: u8) -> ProgramCounter {
        self.load_to(r, n);

        ProgramCounter::Next(2)
    }

    /// ld_r_HL: given register r, load value pointed to by value in register HL into r.
    /// HL is implied and is not included in instruction
    /// 1-byte instruction
    pub fn ld_r_HL(r: u8) -> ProgramCounter {
        let hl = self.reg.HL as u8;
        self.load_to(r, self.ram[hl]);
        
        ProgramCounter::Next(1)
    }

    /// ld_r_IX: given register r and offset d, load contents of IX + offset d to register r.
    /// 3-byte instruction
    pub fn ld_r_IX(r: u8, d: i8) -> ProgramCounter {
        let res = (self.reg.IX as u8) + d ;
        self.load_to(r, res);
        
        ProgramCounter::Next(3)
    }
    
    /// ld_r_IY: given register r and offset d, load contents of IX + offset d to register r.
    /// 3-byte instruction
    pub fn ld_r_IY(r: u8, d: i8) -> ProgramCounter {
        let res = (self.reg.IY as u8) + d;
        self.load_to(r, res);
        
        ProgramCounter::Next(3)
    }

    /// ld_HL_r: Given immediate n. n is loaded into the memory address specified by contents of HL
    /// register.
    /// 2-byte instruction.
    pub fn ld_HR_r(n: u8) -> ProgramCounter {
        self.ram[self.reg.HL] = n;

        ProgramCounter::Next(2)
    }

    /// ld_IX_r: Given immediate n and offset d. n is loaded into the memory address specified by
    /// value in register IX, offset by d.
    /// 4-byte instruction.
    pub fn ld_IX_r(d: usize, n: u8) -> ProgramCounter {
        let addr = self.reg.IX + d;
        self.ram[addr] = n;

        ProgramCounter::Next(4)
    }
    
    /// FD36dn: Given immediate n and offset d. n is loaded into the memory address specified by
    /// value in register IY, offset by d.
    /// 4-byte instruction.
    pub fn ld_IY_r(d: usize, n: u8) -> ProgramCounter {
        let addr = self.reg.IY + d;
        self.ram[addr] = n;

        ProgramCounter::Next(4)
    }
     
    /// ld_A_BC: Load contents of memory location specified by BC register into A (the Accumulator).
    /// 1-byte instruction. Operands are inferred
    pub fn ld_A_BC() -> ProgramCounter {
        self.reg.A = self.ram[self.reg.BC];

        ProgramCounter::Next(1)
    }
    
    /// ld_A_DE: Load contents of memory location specified by DE register into A (Accumulator).
    /// 1-byte instruction.
    pub fn ld_A_DE() -> ProgramCounter {
        self.reg.A = self.ram[self.reg.DE];

        ProgramCounter::Next(1)
    }

    /// ld_A_nn: contents of memory location nn are loaded into A.
    /// 3-byte instruction
    pub fn ld_A_nn(nn: u16) -> ProgramCounter{
        self.reg.A = self.ram[nn as usize];

        ProgramCounter::Next(3)
    }

    /// ld_BC_A: Contents of A are loaded to memory location specified by register BC.
    /// 1-byte instruction.
    pub fn ld_BC_A() -> ProgramCounter {
        self.ram[self.reg.BC] = self.reg.A;

        ProgramCounter::Next(1)
    }

    /// ld_DE_A: Contents of A are loaded to memory location specified by register DE.
    /// 1-byte instruction.
    pub fn ld_DE_A() -> ProgramCounter {
        self.ram[self.reg.DE] = self.reg.A;

        ProgramCounter::Next(1)
    }

    /// ld_nn_A: Contents of A are loaded to memory location specified by operand nn.
    /// 3-byte instruction.
    pub fn ld_nn_A(nn: u16) -> ProgramCounter {
        self.ram[nn as usize] = self.reg.A;

        ProgramCounter::Next(3)
    }

    /// ld_A_I: Contents of I (Interrupt Vector) are loaded to register A.
    /// 2-byte instruction.
    pub fn ld_A_I() -> ProgramCounter {
        self.reg.A = self.reg.I;

        // Configure flags
        self.reg.F &= 0b00000001; // reset all bits except C. N = 0, H = 0
        if self.reg.I < 0 {
            self.reg.F |= SF;           // S = 1 if I < 0
        } else if self.reg.I == 0 {
            self.reg.F |= HF;           // F = 1 if I == 0
        }                           

        self.reg.F |= (self.iff2 << 2); // P = IFF2

        ProgramCounter::Next(2)
    }

    /// ld_A_R: Contents of R (Mem Refresh) are loaded to Accumulator
    /// 2-byte instruction.
    pub fn ld_A_R() -> ProgramCounter {
        self.reg.A = self.reg.R;

        // Configure flags
        self.reg.F &= 0b00000001; // reset all bits except C. N = 0, H = 0
        if self.reg.R < 0 {
            self.reg.F |= SF;           // S = 1 if I < 0
        } else if self.reg.R == 0 {
            self.reg.F |= HF;           // F = 1 if I == 0
        }                           

        self.reg.F |= (self.iff2 << 2); // P = IFF2

        ProgramCounter::Next(2)
    }

    /// ld_I_A: Contents of Accumulator are loaded to I (Interrupt Vector)
    /// 2-byte instruction
    pub fn ld_I_A() -> ProgramCounter {
        self.reg.I = self.reg.A;

        ProgramCounter::Next(2)
    }

    /// ld_R_A: Contents of Accumulator are loaded to R (Mem Refresh)
    /// 2-byte instruction
    pub fn ld_R_A() -> ProgramCounter {
        self.reg.R = self.reg.A;

        ProgramCounter::Next(2)
    }

    /* 16 Bit Load Group */
     
    /// ld_dd_nn: 2-byte integer nn is loaded to dd register pair.
    /// 3-byte instruction
    pub fn ld_dd_nn(dd: u8, nn: u16) -> ProgramCounter {
        load_nn_to(dd, nn);

        ProgramCounter::Next(3)
    }

    /// ld_IX_nn: 2-byte integer nn is loaded to Index Reg IX.
    /// 4-byte instruction
    pub fn ld_IX_nn(nn: u16) -> ProgramCounter {
        self.reg.IX = nn;

        ProgramCounter::Next(4)
    }

    /// ld_IY_nn: 2-byte integer nn is loaded to Index Reg IY.
    /// 4-byte instruction
    pub fn ld_IY_nn(nn: u16) -> ProgramCounter {
        self.reg.IY = nn;

        ProgramCounter::Next(4)
    }

    /// ld_HL_nn: Contents of memory address (nn) are loaded to low-order byte of HL and contents
    /// of next memory address (nn + 1) are loaded to higher-order portion of HL.
    /// 3-byte instruction
    pub fn ld_HL_nn(nn: u16) -> ProgramCounter {
        let content = self.ram[nn as usize] | 
                            (self.ram[(nn + 1) as usize] << 8);
        self.reg.HL = content as usize;

        ProgramCounter::Next(3)
    }

    /// ld_dd_nn: Contents of  memory address (nn) are loaded to lower-byte register pair dd, while
    /// content at (nn+1) are loaded to higher-order byte.
    /// 4-byte instruction
    pub fn ld_dd_nn(dd: u8, nn: u16) {
        let content = self.ram[nn as usize] |
                            (self.ram[(nn + 1) as usize] << 8);
        load_nn_to(dd, content);

        ProgramCounter::Next(4)
    }

    /// ld_IX_nn: Contents of memory address (nn) are loaded to lower-byte of IX, while content at
    /// (nn+1) are loaded to higher-order byte.
    /// 4-byte instruction
    pub fn ld_IX_nn(nn: u16) {
        let content = self.ram[nn as usize] |
                        (self.ram[(nn + 1) as usize] << 8);
        self.reg.IX = content as usize;

        ProgramCounter::Next(4)
    }






















}
