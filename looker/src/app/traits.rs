use paho_mqtt::Message;

pub trait Processor {
    fn process_message(&mut self, msg: Message);
}
