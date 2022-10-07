use std::convert::TryInto;

const BIOS_SIZE: usize = 256 * 1024; // 256kb
const RAM_SIZE: usize = 32 * 1024; // 32kb
const VIDEO_RAM_SIZE: usize = 96 * 1024; // 96kb

const PC_REG: usize = 15; // R15 is PC
const REGISTERS_AMOUNT: usize = 16;

pub struct Cpu {
    // R0.. R15
    registers: [u32; REGISTERS_AMOUNT],
    cpsr: u32, // flags

    rom: [u8; BIOS_SIZE],
    ram: [u8; RAM_SIZE],
    video_ram: [u8; VIDEO_RAM_SIZE],
}

impl Cpu {
    fn read_u32_from_memory(&self, addr: u32) -> u32 {
        let flag = 0xffffff; // turn off upper byte

        let aligned_addr: usize = (addr & flag).try_into().unwrap();

        let mut src : [u8; 4] = [0;4];
        match addr >> 24 {
            0x00 => src.copy_from_slice(&self.rom[aligned_addr .. aligned_addr+3]),
            0x03 => src.copy_from_slice(&self.ram[aligned_addr .. aligned_addr+3]),
            0x06 => src.copy_from_slice(&self.video_ram[aligned_addr .. aligned_addr+3]),
            _ => panic!("wtf address not anything known"),
        };

        return u32::from_le_bytes(src);
    }

    pub fn new() -> Self {
        Self {
            registers: [0; REGISTERS_AMOUNT],
            cpsr: 0,
            rom: [0; BIOS_SIZE],
            ram: [0; RAM_SIZE],
            video_ram: [0; VIDEO_RAM_SIZE],
        }
    }

    pub fn current_opcode(&self) -> u32 {
       self.read_u32_from_memory(self.registers[PC_REG]) 
    }

}
