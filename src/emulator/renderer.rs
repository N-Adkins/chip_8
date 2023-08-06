pub struct Renderer {
    display: [u8; 64 * 32],
}

impl Renderer {
    
    pub fn new() -> Renderer {
        Renderer {
            display: [0; 64 * 32],
        }
    }

    pub fn clear_display(&mut self) {
        self.display.fill(0);
    }

}
