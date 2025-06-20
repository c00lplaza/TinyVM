pub struct Snapshot {}

impl Snapshot {
    pub fn save(&self) { println!("Snapshot saved!"); }
    pub fn load(&self) { println!("Snapshot loaded!"); }
}