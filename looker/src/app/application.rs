use paho_mqtt::Message;
use serde_json::json;

use super::traits::Processor;

use starduck::{traits::UpdateState, Application, SCMessage};

impl Processor for Application {
    fn process_message(&mut self, message: Message) {
        let message_str = message.payload_str();

        info!("message received from topic: `{}`", message.topic());

        let sc_message = match serde_json::from_str::<SCMessage>(&message_str) {
            Ok(k) => k,
            Err(e) => {
                warn!("Unable to deserialize message to SMMessage.");
                warn!("Message recieved:");
                warn!("{}", message.payload_str());
                warn!("Error: {}", e.to_string());
                return;
            }
        };

        info!("device_uuid = {}", sc_message.device_uuid.to_string());
        info!("topic = {}", sc_message.topic);
        info!("values = {}", json!(sc_message.values).to_string());

        // info!(
        //     "\n{}",
        //     serde_json::to_string_pretty(&json!(self.locations)).unwrap()
        // );

        match self.update_state(&sc_message) {
            Ok(_) => info!("Updated state."),
            Err(_) => todo!(),
        };
    }
}
