use bitfield::bitfield;

use crate::cpu::Cpu;

pub enum OpcodeType {
    AluOpcode,
    Branch,
    Unknown,
}

pub trait AluOpcode {
    fn get_second_operand_value(&self, cpu: &Cpu) -> u32;
}

bitfield!{
    pub struct Opcode(u32);
    u32;
    pub get_destination_register, _ : 15,12;
    pub get_first_operand_register, _ : 19,16;
    pub get_set_condition_code, _ : 20;
    pub get_alu_type, _ : 24,21;
    is_immediate_second_operand, _ : 25;
    get_opcode_type, _: 27,26;
}

bitfield!{
    pub struct ImmediateOpcode(u32);
    u32;
    get_immediate, _ : 7,0;
    get_shift_amount, _ : 11,8;
    pub get_destination_register, _ : 15,12;
    pub get_first_operand_register, _ : 19,16;
    pub get_set_condition_code, _ : 20;
    pub get_alu_type, _ : 24,21;
    is_immediate_second_operand, _ : 25;
    get_opcode_type, _: 27,26;
}

bitfield!{
    pub struct RegisterOpcode(u32);
    u32;
    get_second_operand_register, _ : 3,0;
    get_shift_by_register_flag, _ : 4;
    get_shift_type, _ : 6,5;
    get_shift, _ : 11,7; // 8-11 if bit 4 is on 
    pub get_destination_register, _ : 15,12;
    pub get_first_operand_register, _ : 19,16;
    pub get_set_condition_code, _ : 20;
    pub get_alu_type, _ : 24,21;
    is_immediate_second_operand, _ : 25;
    get_opcode_type, _: 27,26;
}

impl Opcode {
    pub fn decode_opcode(opcode: &Opcode) ->OpcodeType {
        match opcode.get_opcode_type() {
            0b00 => OpcodeType::AluOpcode,
            0b10 => OpcodeType::Branch, // actually 0b101 with 25..27
            _ => OpcodeType::Unknown
        }
    }
}

impl AluOpcode for ImmediateOpcode {
    fn get_second_operand_value(&self, _ : &Cpu) -> u32 {
        self.get_immediate() << (2 * self.get_shift_amount())
    }
}

impl AluOpcode for RegisterOpcode {
    fn get_second_operand_value(&self, cpu: &Cpu) -> u32 {
        let mut value = cpu.get_register_value(self.get_second_operand_register().try_into().unwrap());
        let shift_amount = match self.get_shift_type() { 
            1 => cpu.get_register_value((self.get_shift() >> 1).try_into().unwrap()),
            0 => self.get_shift(),
            _ => panic!("wtf how 1 bit is not 0 or 1"),
        };
        
        let value = match self.get_shift_type() {
            0 => value 
            _ => panic!("wtf how 2 bits is not 0-3")
        }
        
        0
    }
}