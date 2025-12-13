use crate::gba::BusAccess;
use super::CPU;
use super::decode::ArmInstructions;


pub fn execute_arm<B: BusAccess>(cpu: &mut CPU, _bus: &mut B, instruction: ArmInstructions) -> u32 {

    match instruction {
        ArmInstructions::MovImm { rd, imm } => {
            // MOV instructiom
            cpu.registers[rd as usize] = imm;
            1
        }
        ArmInstructions::AddImm { rd, rn, imm } => {
            let rn_val = cpu.registers[rn as usize];
            cpu.registers[rd as usize] = rn_val.wrapping_add(imm);
            1
        }
        ArmInstructions::SubImm { rd, rn, imm } => {
            let rn_val = cpu.registers[rn as usize];
            cpu.registers[rd as usize] = rn_val.wrapping_sub(imm);
            1
        }
        ArmInstructions::Unknown => {
            1
        }
    }
}