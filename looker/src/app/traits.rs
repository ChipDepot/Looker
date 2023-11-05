pub(crate) trait Processor {
    fn listen(&mut self, func: fn(String));

    fn process_message(&mut self, message: &str);
}
