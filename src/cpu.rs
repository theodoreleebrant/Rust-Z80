// Defining possible flags in F
const SF: u8 = 0x80;    // 0b10000000
const ZF: u8 = 0x40;    // 0b01000000
const YF: u8 = 0x20;    // 0b00100000
const HF: u8 = 0x10;    // 0b00010000
const XF: u8 = 0x08;    // 0b00001000
const PF: u8 = 0x04;    // 0b00000100
const NF: u8 = 0x02;    // 0b00000010
const CF: u8 = 0x01;    // 0b00000001

// Defining ID for registers
const ID_A: u8 = 0b111; // 8-bit registers
const ID_B: u8 = 0b000;
const ID_C: u8 = 0b001;
const ID_D: u8 = 0b010;
const ID_E: u8 = 0b011;
const ID_H: u8 = 0b100;
const ID_L: u8 = 0b101;
                        
const ID_BC: u8 = 0b000; // 16-bit registers
const ID_DE: u8 = 0b001;
const ID_HL: u8 = 0b010;
const ID_SP: u8 = 0b011;
const ID_IX: u8 = 0b100;
const ID_IY: u8 = 0b101;
const ID_AF: u8 = 0b011; // same as SP but it's okay, won't clash.

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
    pub mut AF: u16
    pub mut BC: u16
    pub mut DE: u16,   
    pub mut HL: u16,   

    // 16-bit combined alternate registers
    pub mut AF_: u16,
    pub mut BC_: u16,   
    pub mut DE_: u16,    
    pub mut HL_: u16,     
    
    // Special registers
    pub mut I : u8,        
    pub mut R : u8,        
    pub mut IX: u16,     
    pub mut IY: u16,     
    pub mut PC: u16,     
    pub mut SP: u16,      
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
    pub fn write_to_reg(&self, reg_id: u8, content: u8) -> bool {
        match reg_id {
            ID_A => self.reg.A = content,
            ID_B => self.reg.B = content,
            ID_C => self.reg.C = content,
            ID_D => self.reg.D = content,
            ID_E => self.reg.E = content,
            ID_H => self.reg.H = content,
            ID_L => self.reg.L = content,
            .. => return false,
        }

        true
    }

    /// load_from takes in a 1-byte register ID reg_id.
    /// Outputs content of register as an Option<u8>, returns None if reg_id is not valid.
    pub fn load_from_reg(&self, reg_id: u8) -> Option<u8> {
        Option<u8> result;

        match reg_id {
            ID_A => result = Some(self.reg.A),
            ID_B => result = Some(self.reg.B),
            ID_C => result = Some(self.reg.C),
            ID_D => result = Some(self.reg.D),
            ID_E => result = Some(self.reg.E),
            ID_H => result = Some(self.reg.H),
            ID_L => result = Some(self.reg.L),
            .. => result = None,
        }

        if result == Some(None) {
            return None;
        }

        result
    }

    /// write_mem_to_reg: Takes in a 1-byte register ID reg_id and an address from which content is
    /// read from. Boolean value is returned.
    pub fn load_mem_to_reg(&self, reg_id: u8, address: u16) -> bool {
        self.write_to_reg(reg_id, self.ram[address as usize])
    }

    /// write_reg_to_mem: Takes in a 1-byte register ID reg_id and an address from which content
    /// will be writen to. If no content is retrieved from register, do nothing.
    pub fn write_reg_to_mem(&self, reg_id: u8, address: u16) {
        match self.load_from_reg(reg_id) {
            Some(val) -> self.ram[address as usize] = val,
            None -> return (),
        }
    }

    /// load_nn_to takes in 2-byte register ID reg_id and content to write. Outputs an optional
    /// boolean value to indicate whether reg_id is valid
    pub fn write_nn_to_reg(&self, reg_id: u8, content: u16) -> bool {
        match reg_id {
            ID_BC => self.reg.BC = content,
            ID_DE => self.reg.DE = content,
            ID_HL => self.reg.HL = content,
            ID_SP => self.reg.SP = content,
            ID_IX => self.reg.IX = content,
            ID_IY => self.reg.IY = content,
            .. => return false,
        }

        true
    }

    /// load_nn_from takes in 2-byte register ID reg_id, and retrieve its content.
    /// Outputs an Option<u16>, or None if reg_id is invalid.
    pub fn load_nn_from_reg(&self, reg_id: u8) -> Option<u16> {
        Option<u16> result;

        match reg_id {
            ID_BC => result = Some(self.reg.BC),
            ID_DE => result = Some(self.reg.DE),
            ID_HL => result = Some(self.reg.HL),
            ID_SP => result = Some(self.reg.SP),
            ID_IX => result = Some(self.reg.IX),
            ID_IY => result = Some(self.reg.IY),
            .. => result = None,
        }

        if result == Some(None) {
            return None;
        }

        result
    }

    /// write_nn_mem_to_reg: Takes in a 2-byte register ID reg_id and a memory address. Contents of
    /// ram[addr] is loaded to lower-order byte, ram[addr + 1] is loaded as higher order byte.
    pub fn write_nn_mem_to_reg(&self, reg_id: u8, addr: u16) -> bool {
        let content = self.ram[addr as usize] |
                        self.ram[(addr + 1) as usize];

        self.write_nn_to_reg(reg_id, content)
    }

    /// write_nn_reg_to_mem: Takes in a 2-byte register ID (reg_id) and a memory address.
    /// lower order byte -> ram[addr]
    /// higher order byte -> ram[addr + 1]
    pub fn write_nn_reg_to_mem(&self, reg_id: u8, addr: u16) {
        match load_nn_from_reg(reg_id) {
            Some(content) => {
                self.ram[addr] = (content & 0x00FF) as u8;
                self.ram[addr + 1] = (content >> 8) as u8;
            },
            None => return (),
        }
    }
    
    /// push: Takes in a 2-byte register ID (reg_id). 

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
       match self.load_from_reg(ry) {
           None => (), // FIXME: How do i handle error here
           Some(value) => self.write_to_reg(rx, value),
       }

       ProgramCounter::Next(1) // Increment the program counter
    }
           
    /// ld_r_n: given register r and immediate u8 value n. load n into r
    /// 2-byte instruction
    pub fn ld_r_n(r: u8, n: u8) -> ProgramCounter {
        self.write_to_reg(r, n);

        ProgramCounter::Next(2)
    }

    /// ld_r_HL: given register r, load value pointed to by value in register HL into r.
    /// HL is implied and is not included in instruction
    /// 1-byte instruction
    pub fn ld_r_HL(r: u8) -> ProgramCounter {
        self.write_mem_to_reg(r, self.reg.HL);
        
        ProgramCounter::Next(1)
    }

    /// ld_r_IX: given register r and offset d, load contents of IX + offset d to register r.
    /// 3-byte instruction
    pub fn ld_r_IX(r: u8, d: i16) -> ProgramCounter {
        self.write_mem_to_reg(r, self.reg.IX + d);
        
        ProgramCounter::Next(3)
    }
    
    /// ld_r_IY: given register r and offset d, load contents of IX + offset d to register r.
    /// 3-byte instruction
    pub fn ld_r_IY(r: u8, d: i16) -> ProgramCounter {
        self.write_mem_to_reg(r, self.reg.IY + d);

        ProgramCounter::Next(3)
    }

    /// ld_HL_r: Given register r. contents of r are loaded into the memory address specified by contents of HL
    /// register.
    /// 2-byte instruction.
    pub fn ld_HL_r(r: u8) -> ProgramCounter {
        self.write_reg_to_mem(r, self.reg.HL);

        ProgramCounter::Next(2)
    }

    /// ld_IX_r: Given immediate n and offset d. n is loaded into the memory address specified by
    /// value in register IX, offset by d.
    /// 4-byte instruction.
    pub fn ld_IX_r(r: u8, d: i16) -> ProgramCounter {
        self.write_reg_to_mem(r, self.reg.IX + d);

        ProgramCounter::Next(4)
    }
    
    /// FD36dn: Given immediate n and offset d. n is loaded into the memory address specified by
    /// value in register IY, offset by d.
    /// 4-byte instruction.
    pub fn ld_IY_r(r: u8, d: i16) -> ProgramCounter {
        self.write_reg_to_mem(r, self.reg.IY + d);

        ProgramCounter::Next(4)
    }
     
    /// ld_A_BC: Load contents of memory location specified by BC register into A (the Accumulator).
    /// 1-byte instruction. Operands are inferred
    pub fn ld_A_BC() -> ProgramCounter {
        self.write_mem_to_reg(ID_A, self.reg.BC);

        ProgramCounter::Next(1)
    }
    
    /// ld_A_DE: Load contents of memory location specified by DE register into A (Accumulator).
    /// 1-byte instruction.
    pub fn ld_A_DE() -> ProgramCounter {
        self.write_mem_to_reg(ID_A, self.reg.DE);

        ProgramCounter::Next(1)
    }

    /// ld_A_nn: contents of memory location nn are loaded into A.
    /// 3-byte instruction
    pub fn ld_A_nn(nn: u16) -> ProgramCounter{
        self.write_mem_to_reg(ID_A, nn);

        ProgramCounter::Next(3)
    }

    /// ld_BC_A: Contents of A are loaded to memory location specified by register BC.
    /// 1-byte instruction.
    pub fn ld_BC_A() -> ProgramCounter {
        self.write_reg_to_mem(ID_A, self.reg.BC);

        ProgramCounter::Next(1)
    }

    /// ld_DE_A: Contents of A are loaded to memory location specified by register DE.
    /// 1-byte instruction.
    pub fn ld_DE_A() -> ProgramCounter {
        self.write_reg_to_mem(ID_A, self.reg.DE);

        ProgramCounter::Next(1)
    }

    /// ld_nn_A: Contents of A are loaded to memory location specified by operand nn.
    /// 3-byte instruction.
    pub fn ld_nn_A(nn: u16) -> ProgramCounter {
        self.write_reg_to_mem(ID_A, nn);

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
        self.write_nn_to_reg(dd, nn);

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
    pub fn ld_HL_addr_nn(nn: u16) -> ProgramCounter {
        self.write_nn_mem_to_reg(ID_HL, nn);

        ProgramCounter::Next(3)
    }

    /// ld_dd_nn: Contents of  memory address (nn) are loaded to lower-byte register pair dd, while
    /// content at (nn+1) are loaded to higher-order byte.
    /// 4-byte instruction
    pub fn ld_dd_addr_nn(dd: u8, nn: u16) -> ProgramCounter{
        self.write_nn_mem_to_reg(dd, nn);

        ProgramCounter::Next(4)
    }

    /// ld_IX_addr_nn: Contents of memory address (nn) are loaded to lower-byte of IX, while content at
    /// (nn+1) are loaded to higher-order byte.
    /// 4-byte instruction
    pub fn ld_IX_addr_nn(nn: u16) -> ProgramCounter {
        self.write_nn_mem_to_reg(ID_IX, nn);

        ProgramCounter::Next(4)
    }

    /// ld_IY_addr_nn: Contents of memory address (nn) are loaded to lower-byte of IY, while content at
    /// (nn+1) are loaded to higher-order byte.
    /// 4-byte instruction
    pub fn ld_IY_addr_nn(nn: u16) -> ProgramCounter {
        self.write_nn_mem_to_reg(ID_IY, nn);

        ProgramCounter::Next(4)
    }

    /// ld_addr_nn_HL: Lower-order byte of HL is loaded to memory address (nn), higher-order byte of HL is
    /// loaded to memory address (nn + 1)
    /// 3-byte instruction
    pub fn ld_addr_nn_HL(nn: u16) -> ProgramCounter {
        self.write_nn_reg_to_mem(ID_HL, nn);

        ProgramCounter::Next(3)
    }

    /// ld_addr_nn_dd: Lower-order byte of register pair dd is loaded to memory address (nn),
    /// higher-order byte is loaded to address (n + 1)
    /// 4-byte instruction
    pub fn ld_addr_nn_dd(dd: u8, nn: u16) -> ProgramCounter {
        self.write_nn_reg_to_mem(dd, nn);

        ProgramCounter::Next(4)
    }

    /// ld_addr_nn_IX: Lower_order byte of IX is loaded to mem address (nn), higher-order byte is
    /// loaded to memory address (nn + 1)
    /// 4-byte instruction
    pub fn ld_addr_nn_IX(nn: u16) -> ProgramCounter {
        self.write_nn_reg_to_mem(ID_IX, nn);

        ProgramCounter::Next(4)
    }

    /// ld_addr_nn_IY: Lower_order byte of IY is loaded to mem address (nn), higher-order byte is
    /// loaded to memory address (nn + 1)
    /// 4-byte instruction
    pub fn ld_addr_nn_IY(nn: u16) -> ProgramCounter {
        self.write_nn_reg_to_mem(ID_IY, nn);

        ProgramCounter::Next(4)
    }
    
    /// ld_SP_HL: Contents of register pair HL are loaded to SP
    /// 1-byte instruction
    pub fn ld_SP_HL() -> ProgramCounter {
        self.reg.SP = self.reg.HL;

        ProgramCounter::Next(1)
    }

    /// ld_SP_IX: Contents of register pair IX are loaded to SP
    /// 2-byte instruction
    pub fn ld_SP_IX() -> ProgramCounter {
        self.reg.SP = self.reg.IX;

        ProgramCounter::Next(2)
    }

    /// ld_SP_IY: Contents of register pair IY are loaded to SP
    /// 2-byte instruction
    pub fn ld_SP_IY() -> ProgramCounter {
        self.reg.SP = self.reg.IY;

        ProgramCounter::Next(2)
    }

    /// push_qq: Contents of register pair qq are pushed to external memory LIFO stack. Stack
    /// pointer hold 16-bit addr of top of stack.


















}
