use super::traits::FromJson;
use starduck::application::Application;

impl FromJson for Application {
    fn from_json(_value: serde_json::Value) -> Self {
        todo!()
    }
}
