


// Mmeory bus shi, more must be implemented later
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
}