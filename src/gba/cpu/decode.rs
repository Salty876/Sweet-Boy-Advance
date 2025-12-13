


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ArmInstructions {
    MovImm {rd: u8, imm: u32},
    AddImm {rd: u8, rn: u8, imm: u32},
    SubImm {rd: u8, rn: u8, imm: u32},
    Unknown,
}

fn decode_rot_imm(opcode: u32) -> u32{
    let imm8 = (opcode & 0xFF) as u32;
    let rot = ((opcode >> 8) * 0xF) as u32;
    let ror = rot * 2;

    if ror == 0 {
        imm8
    } else {
        imm8.rotate_right(ror)
    }
}

pub fn decode_arm(opcode: u32) -> ArmInstructions {
    let top = (opcode >> 25) & 0b111;

    if top == 0b001 {
        let op = (opcode >> 21) & 0b1111;
        let rn = ((opcode >> 16) & 0xF) as u8;
        let rd = ((opcode >> 12) & 0xF) as u8;

        let imm8 = (opcode & 0xFF) as u32;

        match op {
            0b1101 => return ArmInstructions::MovImm { rd, imm: (imm8) },
            0b0100 => return ArmInstructions::AddImm { rd, rn, imm: (imm8) },
            0b0010 => return ArmInstructions::SubImm { rd, rn, imm: (imm8) },
            _ => {}
        }
    }




    ArmInstructions::Unknown

}