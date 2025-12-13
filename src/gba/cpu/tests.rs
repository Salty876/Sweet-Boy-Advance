#[cfg(test)]
mod tests{
    
    use crate::gba::{MemoryBus, CPU};

    #[test]
    fn cpu_steps_through_rom_arm(){
        let mut bus = MemoryBus::new();
        
        // 3 instructions
        bus.load_rom(vec![
            0x00, 0x00, 0x00, 0x00, // at 0x0800_0000
            0x11, 0x11, 0x11, 0x11, // at 0x0800_0004
            0x22, 0x22, 0x22, 0x22, // at 0x0800_0008
        ]);

        let mut cpu = CPU::new();

        assert_eq!(cpu.pc(), 0x0800_0000);

        cpu.step(&mut bus);
        assert_eq!(cpu.pc(), 0x0800_0004);

        cpu.step(&mut bus);
        assert_eq!(cpu.pc(), 0x0800_0008);

        cpu.step(&mut bus);
        assert_eq!(cpu.pc(), 0x0800_000C);

    }
}

#[cfg(test)]
mod exec_tests {
   use crate::gba::{CPU, MemoryBus};

    // Helps create immediate instructions
    fn arm_dp_imm(op: u8, rn: u8, rd: u8, rot: u8, imm8: u8) -> u32 {
        let cond = 0b1110u32 << 28;          // AL
        let dp_imm = 0b001u32 << 25;         // data processing immediate
        let opcode = (op as u32) << 21;
        let rn_bits = (rn as u32) << 16;
        let rd_bits = (rd as u32) << 12;
        let rot_bits = ((rot as u32) & 0xF) << 8;
        let imm_bits = imm8 as u32;
        cond | dp_imm | opcode | rn_bits | rd_bits | rot_bits | imm_bits
    }

    pub fn arm_mov_imm(rd: u8, rot: u8, imm8: u8) -> u32 {
        arm_dp_imm(0b1101, 0, rd, rot, imm8)
    }

    fn arm_add_imm(rd: u8, rn: u8, rot: u8, imm8: u8) -> u32 {
        arm_dp_imm(0b0100, rn, rd, rot, imm8)
    }

    fn arm_sub_imm(rd: u8, rn: u8, rot: u8, imm8: u8) -> u32 {
        arm_dp_imm(0b0010, rn, rd, rot, imm8)
    }

    #[test]
    fn mov_imm_sets_register() {
        let mut bus = MemoryBus::new();

        let instr = arm_mov_imm(0, 0, 7); // MOV R0, #7
        let bytes = instr.to_le_bytes().to_vec();
        bus.load_rom(bytes);

        let mut cpu = CPU::new();
        cpu.set_pc(0x0800_0000);

        cpu.step(&mut bus);

        assert_eq!(cpu.registers[0], 7);
        assert_eq!(cpu.pc(), 0x0800_0004);
    }

    #[test]
    fn add_imm_works() {
        let mut bus = MemoryBus::new();

        // Program:
        // MOV R1, #10
        // ADD R0, R1, #5
        let i1 = arm_mov_imm(1, 0, 10);
        let i2 = arm_add_imm(0, 1, 0, 5);

        let mut rom = Vec::new();
        rom.extend_from_slice(&i1.to_le_bytes());
        rom.extend_from_slice(&i2.to_le_bytes());

        bus.load_rom(rom);

        let mut cpu = CPU::new();
        cpu.set_pc(0x0800_0000);

        cpu.step(&mut bus);
        cpu.step(&mut bus);

        assert_eq!(cpu.registers[1], 10);
        assert_eq!(cpu.registers[0], 15);
    }

    #[test]
    fn sub_imm_works() {
        let mut bus = MemoryBus::new();

        // MOV R1, #10
        // SUB R0, R1, #3  => 7
        let i1 = arm_mov_imm(1, 0, 10);
        let i2 = arm_sub_imm(0, 1, 0, 3);

        let mut rom = Vec::new();
        rom.extend_from_slice(&i1.to_le_bytes());
        rom.extend_from_slice(&i2.to_le_bytes());
        bus.load_rom(rom);

        let mut cpu = CPU::new();
        cpu.set_pc(0x0800_0000);

        cpu.step(&mut bus);
        cpu.step(&mut bus);

        assert_eq!(cpu.registers[0], 7);
    }

    #[test]
    fn mov_imm_rotate_is_applied() {
        let mut bus = MemoryBus::new();

        // MOV R0, # (imm8=1 rotated right by 2) = 0x4000_0000
        // rot field is in units of 2 bits, so rot=1 => rotate by 2.
        let i1 = arm_mov_imm(0, 1, 1);

        bus.load_rom(i1.to_le_bytes().to_vec());

        let mut cpu = CPU::new();
        cpu.set_pc(0x0800_0000);

        cpu.step(&mut bus);

        assert_eq!(cpu.registers[0], 0x4000_0000);
    }



}


