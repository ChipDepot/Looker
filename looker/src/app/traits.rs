use serde_json::Value;

pub(super) trait FromJson {
    fn from_json(value: Value) -> Self;
}
