pub mod memory;


struct gba {
    memory: u8
}

impl gba {
    fn new() -> gba {
       let instance = gba {
        memory: 9
       };

       return  instance;
    }

    
}