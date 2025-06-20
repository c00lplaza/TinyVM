pub struct Memory { pub data: Vec<u8> }

impl Memory {
    pub fn new(size: usize) -> Self { Memory { data: vec![0; size] } }
}