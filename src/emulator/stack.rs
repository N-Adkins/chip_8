pub struct Stack {
    sp: u8,
    data: [u16; 16],
}

impl Stack {
    
    pub fn new() -> Stack {
        Stack {
            sp: 0,
            data: [0; 16],
        }
    }

    pub fn push(&mut self, value: u16) {
        self.data[self.sp as usize] = value;
        self.sp += 1;
    }

    pub fn pop(&mut self) -> u16 {
        self.sp -= 1;
        self.data[self.sp as usize]
    }

}


