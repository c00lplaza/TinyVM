pub trait Plugin {
    fn name(&self) -> &str;
    fn run(&self);
}