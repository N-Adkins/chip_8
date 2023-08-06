pub struct Memory {
    data: [u8; 0xFFF],
}

impl Memory {
    
    pub fn new() -> Memory {
        Memory {
            data: [0; 0xFFF],
        }
    }

    pub fn read_u8(&self, address: u16) -> u8 {
        self.data[address as usize]
    }

    pub fn read_u16(&self, address: u16) -> u16 {
        // Big endian so first byte goes in top 8 bits
        ((self.data[address as usize] as u16) << 8) | (self.data[(address + 1) as usize]) as u16
    }

    pub fn load_rom(&mut self, data: &Vec<u8>) {
        if 0x200 + data.len() <= 0xFFF {
            self.data[0x200..(0x200 + data.len())].copy_from_slice(&data);
        } else {
            panic!("Failed to road ROM: File too large");
        }
    }

}
