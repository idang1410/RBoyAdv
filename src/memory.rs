use crate::opcodes::Opcode;

const BIOS_SIZE: usize = 16 * 1024; // 16kb
const RAM_SIZE: usize = 256 * 1024; // 256kb
const VIDEO_RAM_SIZE: usize = 96 * 1024; // 96kb

pub struct Memory {
    rom: [u8; BIOS_SIZE],
    ram: [u8; RAM_SIZE],
    video_ram: [u8; VIDEO_RAM_SIZE],
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            rom: [0; BIOS_SIZE],
            ram: [0; RAM_SIZE],
            video_ram: [0; VIDEO_RAM_SIZE],
        }
    }

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
 
    pub fn current_opcode(&self, addr: u32) -> Opcode {
       Opcode(self.read_u32_from_memory(addr))
    }
}