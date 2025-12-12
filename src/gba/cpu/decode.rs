


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ArmInstructions {
    MovImm {rd: u8, imm: u32},
    Unknown,
}

pub fn decode_arm(opcode: u32) -> ArmInstructions {
    let op = (opcode >> 25) & 0b111;

    // FINISH TRMW




    ArmInstructions::Unknown

}