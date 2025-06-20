pub struct CPU {}

impl CPU {
    pub fn new() -> Self { CPU {} }
    pub fn step(&mut self) { println!("CPU step"); }
}