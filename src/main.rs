extern crate bitfield;

mod cpu;
mod opcodes;
mod alu;


fn main() {
    let cpu = cpu::Cpu::new();
    let current_opcode = cpu.current_opcode();
    let opcode = opcodes::Opcode(current_opcode);

    match opcodes::decode_opcode(&opcode) {
        opcodes::OpcodeType::AluOpcode => alu::compute(&opcode),
        _ => panic!("Unimplemented opcode")
    };
}
