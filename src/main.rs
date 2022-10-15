extern crate bitfield;

mod cpu;
mod opcodes;
mod alu;
mod memory;


fn main() {
    let cpu = cpu::Cpu::new();
    let memory = memory::Memory::new();
    let current_opcode = memory.current_opcode(cpu.get_current_opcode_addr());

    match opcodes::Opcode::decode_opcode(&current_opcode) {
        opcodes::OpcodeType::AluOpcode => alu::compute(&current_opcode),
        _ => panic!("Unimplemented opcode")
    };
}
