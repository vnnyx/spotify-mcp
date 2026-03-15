use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct AddToQueueParams {
    /// The Spotify URI of the track to add to queue
    pub uri: String,
    /// The ID of the device to target
    pub device_id: Option<String>,
}
