use crate::gba::BusAccess;

pub mod decode;
pub mod arm;

pub const STACK_POINTER_REG:usize = 13;
pub const LINK_REG:usize = 14; 
pub const PROGRAM_COUNTER_REG:usize = 15;

pub struct CPU {
    // 16 registers
    pub registers: [u32; 16],

    // cpsr register
    pub cpsr: u32,

    // Weather in arm mode or thumb mode
    pub thumbMode: bool,

    // total cycles done (needed for timing shi)
    pub cycles: u64,
}

impl CPU {
    pub fn new() -> Self{
        let mut cpu = Self {
            registers: [0; 16],
            cpsr: 0,
            thumbMode: false,
            cycles: 0,
        };

        // Set pc to rom boot, chanmge it to 0 if BIOS gets added
        cpu.set_pc(0x0800_0000);

        cpu
    }

    pub fn pc(&self) -> u32 {
        self.registers[PROGRAM_COUNTER_REG]
    }

    pub fn set_pc(&mut self, val:u32) {
        self.registers[PROGRAM_COUNTER_REG] = val;
    }

    pub fn sp(&self) -> u32 {
        self.registers[STACK_POINTER_REG]
    }
    
    pub fn set_sp(&mut self, val:u32) {
        self.registers[STACK_POINTER_REG] = val;
    }

    pub fn lr(&self) -> u32 {
        self.registers[LINK_REG]
    }
    
    pub fn set_lr(&mut self, val:u32) {
        self.registers[LINK_REG] = val;
    }


    // Step through inbstructions

    pub fn step<B: BusAccess>(&mut self, bus: &mut B) -> u32 {
        
        // check cpu mode
        if self.thumbMode {
            // Thumb 16 bit instructionms happen here
            let opcode = bus.read16(self.pc());

            // up 2 for now
            self.set_pc(self.pc().wrapping_add(2));


            // TODO: decoed and execute thumb
            let cycles = 1;
            self.cycles += cycles as u64;
            cycles
    
        }else{
            // Arm 32 bitr instryctions here
            let opcode = bus.read32(self.pc());

            // up 2 for now
            self.set_pc(self.pc().wrapping_add(4));


            // TODO: decoed and execute thumb
            let cycles = 1;
            self.cycles += cycles as u64;
            cycles
        }
    }

}




#[cfg(test)]
mod tests{
    use super::*;
    use crate::gba::{BusAccess, MemoryBus, cpu};

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