pub struct Instruction {
    pub raw: u16,
}

impl Instruction {

    pub fn new(raw: u16) -> Instruction {
        Instruction {
            raw,
        }
    }

    pub fn nnn(&self) -> u16 {
        self.raw & 0x0FFF
    }

    pub fn n(&self) -> u16 {
        self.raw & 0x000F
    }

    pub fn x(&self) -> usize {
        ((self.raw & 0x0F00) >> 8) as usize 
    }

    pub fn y(&self) -> usize {
        ((self.raw & 0x00F0) >> 4) as usize
    }

    pub fn kk(&self) -> u8 {
        (self.raw & 0x00FF) as u8
    }

}
