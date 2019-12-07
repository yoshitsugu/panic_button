use lambda_runtime::{error::HandlerError, lambda, Context};
use serde_json::Value;
use rusoto_core::region::Region::ApNortheast1;
use rusoto_iot_data::{IotDataClient, PublishRequest};
use rusoto_iot_data::IotData;
use bytes::Bytes;

fn main() {
    lambda!(handler)
}

fn handler(
    event: Value,
    _: Context,
) -> Result<Value, HandlerError> {
    let client = IotDataClient::new(ApNortheast1);
    let payload = PublishRequest {
        payload: Some(Bytes::from(&b"{ \"message\": \"panic button is pushed\" }"[..])),
        qos: None,
        topic: "panic_button".to_string()
    };
    client.publish(payload).sync().unwrap();
    Ok(event)
}
