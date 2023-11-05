use super::traits::Processor;
use starduck::application::Application;

impl Processor for Application {
    fn listen(&mut self, _func: fn(String)) {
        println!("hting");
    }

    fn process_message(&mut self, message: &str) {
        /*
        {
            "deviceUUID": "1",
            "topic": "temperatura",
            "timeStamp": "30-04-2023 10:39:02",
            "values": {
                "temperature": 10.0,
                "co2": 15.0,
                "location": "AP2"
            },
            "status": "OK",
            "alert": false
        },
        */
    }
}
