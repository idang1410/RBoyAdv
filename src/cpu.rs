const PC_REG: usize = 15; // R15 is PC
const REGISTERS_AMOUNT: usize = 16;

pub struct Cpu {
    // R0.. R15
    registers: [u32; REGISTERS_AMOUNT],
    cpsr: u32, // flags
}

impl Cpu {
   pub fn new() -> Self {
        Self {
            registers: [0; REGISTERS_AMOUNT],
            cpsr: 0,
        }
    }
   
    pub fn get_register_value(&self, register: usize) -> u32 {
        self.registers[register]
    }

    pub fn get_current_opcode_addr(&self) -> u32 {
        self.registers[PC_REG]
    }
}
