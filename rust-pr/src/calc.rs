pub trait Calculator {
    fn add(&self) -> u8;
    fn sub(&self) -> u8;
    fn new(x: u8, y: u8) -> Numbers;
}

pub struct Numbers {
    x: u8,
    y: u8,
}

impl Calculator for Numbers {
    fn new(x: u8, y: u8) -> Numbers {
        Numbers { x, y }
    }

    fn add(&self) -> u8 {
        return self.x + self.y;
    }

    fn sub(&self) -> u8 {
        return if self.x > self.y { self.x - self.y } else {self.y - self.x};
    }
}