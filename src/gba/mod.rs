pub mod memory;
mod bus;


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