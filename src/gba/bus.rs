


// Mmeory bus shi, more must be implemented later


pub trait BusAccess {
    fn read8(&mut self, addr:u32) -> u8;
    fn read16(&mut self, addr:u32) -> u16;
    fn read32(&mut self, addr:u32) -> u32;

    fn write8(&mut self, addr:u32, val:u8);
    fn write16(&mut self, addr:u32, val:u16);
    fn write32(&mut self, addr:u32, val:u32);
}



pub struct MemoryBus {
    pub EWRAM: [u8; 256 * 1024],
    pub rom: Vec<u8>
}

impl MemoryBus{
    pub fn new() -> Self {

        Self {
            EWRAM: [0; 1024 * 256],
            rom: Vec::new()
         }
    }


    pub fn load_rom(&mut self, rom:Vec<u8>){
        self.rom = rom;
    }

    pub fn map_ewram_index(&self, addr: u32) -> Option<usize> {
        const EWRAM_BASE:u32 = 0x02000000;
        const EWRAM_SIZE:u32 = 1024 * 256;

        if addr >= EWRAM_BASE && addr < EWRAM_SIZE + EWRAM_BASE {
            Some((addr - EWRAM_BASE) as usize)
        }else{
            // for the time being
            None
        }
    }


    pub fn map_rom_index(&self, addr:u32) -> Option<usize>{
        const ROM_BASE:u32 = 0x08000000;

        if addr < ROM_BASE {
            return None;
        }

        let offset = (addr.wrapping_sub(ROM_BASE)) as usize;
        
        if offset < self.rom.len(){
            Some(offset)
        }else{
            None
        }
    }
}

impl BusAccess for MemoryBus{
    fn read8(&mut self, addr:u32) -> u8 {
        // ewram 
        if let Some(index) = self.map_ewram_index(addr) && let Some(ewram_val) = self.EWRAM.get(index){
            println!("GOT EWRAM INDEX");
            return *ewram_val
        }

        // Rom
        if let Some(index)  = self.map_rom_index(addr) && let Some(rompak) = self.rom.get(index){
            println!("GOT rompak INDEX, {}", index);
            return *rompak;
        }

        // Does nothing for the time now
        0
    }

    fn read16(&mut self, addr:u32) -> u16 {
        let b0 = self.read8(addr) as u16;
        let b1 = self.read8(addr.wrapping_add(1)) as u16;

        b0 | (b1 << 8)
    }

    fn read32(&mut self, addr:u32) -> u32 {
        let b0 = self.read8(addr) as u32;
        let b1 = self.read8(addr.wrapping_add(1)) as u32;
        let b2 = self.read8(addr.wrapping_add(2)) as u32;
        let b3 = self.read8(addr.wrapping_add(3)) as u32;

        b0 | (b1 << 8) | (b2 << 16) | (b3 << 24)

    }

    
    fn write8(&mut self, addr:u32, val:u8) {

        // Ewram writes
        if let Some(index) = self.map_ewram_index(addr) {
            self.EWRAM[index] = val;
        }

        if let Some(index) = self.map_rom_index(addr) {
            // Ignore rom writes for now

            return ;
        }

    }

    fn write16(&mut self, addr:u32, val:u16) {
        let b0 = (val & 0x00FF) as u8;
        let b1 = (val >> 8) as u8;
        self.write8(addr, b0);
        self.write8(addr.wrapping_add(1), b1);
    }

    fn write32(&mut self, addr:u32, val:u32) {
        let b0 = (val & 0x000000FF) as u8;
        let b1 = ((val << 8) & 0xFF) as u8;
        let b2 = ((val << 16) & 0xFF) as u8;
        let b3 = ((val << 24) & 0xFF) as u8;

        self.write8(addr, b0);
        self.write8(addr.wrapping_add(1), b1);
        self.write8(addr.wrapping_add(2), b2);
        self.write8(addr.wrapping_add(3), b3);


    }
}



#[cfg(test)]
mod tests {
    

    use super::*;

    // must create a bus rq
    fn make_bus_wit_rom() -> MemoryBus {
        let mut bus = MemoryBus::new();

        bus.load_rom(vec![0xEE, 0xAA, 0xFF, 0xDD]);
        println!("{:?}", bus.rom);
        bus
    }

    #[test]
    fn ewram_read_write_8() {
        let mut bus = make_bus_wit_rom();

        // ewaram base addy
        let addr =  0x02000000;


        // Need to write byte and get it back
        bus.write8(addr, 0xBB);
        let val = bus.read8(addr);
        assert_eq!(val, 0xBB);

        // Offset of doom and despair
        bus.write8(addr + 98, 0x32);
        let val2 = bus.read8(addr + 98);
        assert_eq!(val2, 0x32);
    }

    #[test]
    fn rom_read_8_16_32(){
        let mut bus = make_bus_wit_rom();

        let base = 0x08000000;

        assert_eq!(bus.read8(base), 0xEE);
        println!("{} vs {}", bus.read8(base.wrapping_add(1)), 0xAA);
        assert_eq!(bus.read8(base.wrapping_add(1)), 0xAA);
        assert_eq!(bus.read8(base + 2), 0xFF);
        assert_eq!(bus.read8(base + 3), 0xDD);

        // 16 bit read
        let h = bus.read16(base);
        assert_eq!(h, 0xAAEE);

        // 32 bit read (a word fn)
        let w = bus.read32(base);
        assert_eq!(w, 0xDDFFAAEE);
    }

    #[test]
    fn rom_is_ONLY_read(){
        let mut bus = make_bus_wit_rom();

        let base = 0x08000000;

        bus.write8(base, 0xFF);
        assert_eq!(bus.read8(base), 0xEE);
    }

}