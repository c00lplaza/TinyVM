use std::fs::File;
use std::io::{self, Read};

pub struct Disk { pub data: Vec<u8> }

impl Disk {
    pub fn from_iso(_path: &str) -> std::io::Result<Self> {
        Ok(Disk { data: vec![] })
    }
    // Add methods for reading sectors, etc.
}