use bitfield::bitfield;

pub enum OpcodeType {
    AluOpcode,
    Branch,
    Unknown,
}

bitfield!{
    pub struct Opcode(u32);
    u32;
    pub get_alu_type, _ : 24,21;
    get_opcode_type, _: 27,26;
}

pub fn decode_opcode(opcode: &Opcode) ->OpcodeType {
    match opcode.get_opcode_type() {
        0b00 => OpcodeType::AluOpcode,
        0b10 => OpcodeType::Branch, // actually 0b101 with 25..27
        _ => OpcodeType::Unknown
    }
}
