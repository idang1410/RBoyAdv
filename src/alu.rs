use crate::opcodes::Opcode;

enum AluOpcode {
    AND,
    EOR,
    SUB,
    RSB,
    ADD,
    ADC,
    SBC,
    RSC,
    TST,
    TEQ,
    CMP,
    CMN,
    ORR,
    MOV,
    BIC,
    MVN,
}

fn get_opcode_type(opcode: &Opcode) -> AluOpcode {
     match opcode.get_alu_type() {
            0x0 => AluOpcode::AND,
            0x1 => AluOpcode::EOR,
            0x2 => AluOpcode::SUB,
            0x3 => AluOpcode::RSB,
            0x4 => AluOpcode::ADD,
            0x5 => AluOpcode::ADC,
            0x6 => AluOpcode::SBC,
            0x7 => AluOpcode::RSC,
            0x8 => AluOpcode::TST,
            0x9 => AluOpcode::TEQ,
            0xA => AluOpcode::CMP,
            0xB => AluOpcode::CMN,
            0xC => AluOpcode::ORR,
            0xD => AluOpcode::MOV,
            0xE => AluOpcode::BIC,
            0xF => AluOpcode::MVN,
            _ => panic!("Expected only uint8 values")
     }
}

pub fn compute(opcode: &Opcode) {
    let alu_type = get_opcode_type(opcode);
}