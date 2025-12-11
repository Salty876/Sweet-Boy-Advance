


// Mmeory bus shi, more must be implemented later

pub trait busAccess {
    fn read8(&mut self, addr:u32) -> u8;
    fn read16(&mut self, addr:u32) -> u16;
    fn read32(&mut self, addr:u32) -> u32;

    fn write8(&mut self, addr:u32, val:u8);
    fn write16(&mut self, addr:u32, val:u16);
    fn write32(&mut self, addr:u32, val:u32);


}



pub struct memoryBus {
    pub EWRAM: [u8; 256 * 1024],
    pub rom: Vec<u8>
}

impl memoryBus{
    fn new() -> self {
        let mem = memoryBus {
            EWRAM: [0; 1024 * 256],
            rom: Vec::new()
        };

        return mem;
    }


    pub fn load_rom(&mut self, rom:Vec<u8>){
        self.rom = rom;
    }

    pub fn map_ewram_index(addr: u32) -> Option<usize> {
        const EWRAM_BASE:u32 = 0x02000000;
        const EWRAM_SIZE:u32 = 1024 * 256;

        if (addr >= EWRAM_BASE && addr < EWRAM_SIZE + EWRAM_BASE) {
            Some((addr - EWRAM_BASE) as usize)
        }else{
            // for the time being
            None
        }
    }


    pub fn map_rom_undex(&self, addr:u32) -> Option<usize>{
        const ROM_BASE:u32 = 0x08000000;

        if (addr > ROM_BASE){
            None
        }

        let offset = (addr - ROM_BASE) as usize;
        if (offset < self.rom.len()){
            Some((offset))
        }else{
            None
        }
    }
}

impl busAccess for memoryBus{
    fn read8(&mut self, addr:u32) -> u8 {
        // ewram 
        if let Some(index) = self.map_ewram_index(addr){
            return self.EWRAM[index];
        }

        
    }

    fn read16(&mut self, addr:u32) -> u16 {
        
    }

    fn read32(&mut self, addr:u32) -> u32 {
        
    }

    fn write16(&mut self, addr:u32, val:u16) {
        
    }

    fn write32(&mut self, addr:u32, val:u32) {
        
    }

    fn write8(&mut self, addr:u32, val:u8) {
        
    }
}