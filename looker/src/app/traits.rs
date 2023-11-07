pub trait Processor {
    fn process_message(&mut self, message: &str);
}
