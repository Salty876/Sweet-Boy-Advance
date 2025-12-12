pub mod memory;
pub mod bus;
pub mod cpu;



pub use bus::{MemoryBus, BusAccess};
pub use cpu::CPU;



pub struct gba {
    memoryBus: MemoryBus,
    cpu: CPU,
}

impl gba {
    pub fn new() -> gba {
       let instance = gba {
        memoryBus: bus::MemoryBus::new(),
        cpu: CPU::new(),
       };

       return  instance;
    }

    pub fn load_rom(&mut self, data: Vec<u8>) {
        self.memoryBus.load_rom(data);

        // Set cpu to rom base for now since (in caase i add bios)
        self.cpu.set_pc(0x0800_0000);
    }

    pub fn run_frame(&mut self) {
        // Random number of steps
        for _ in 0..10_00 {
            self.cpu.step(&mut self.memoryBus);
        }
    }
    
}