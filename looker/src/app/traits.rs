pub(crate) trait Listener {
    fn listen(&mut self, func: fn(String));

    fn process_message(&mut self, message: &str);
}
