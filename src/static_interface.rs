use futures_util::{sink::SinkExt, StreamExt};
use serde_json::{to_string, to_value, from_value, from_str};
use tokio_tungstenite::{connect_async, tungstenite::Message};

use union_structs::{StaticRequest, StaticResponse};

pub async fn get_static(url: &str) -> String {
    let (mut ws_stream, _) = connect_async("ws://127.0.0.1:2978")
        .await
        .expect("Did not connect");
    let message: String = to_string(
        &to_value(StaticRequest::new(String::from(url)))
            .expect("Failed to convert StaticRequest into JSON"),
    )
    .expect("Failed to convert JSON into String.");
    ws_stream
        .send(Message::from(message))
        .await
        .expect("WS send error");
    let returned_message = ws_stream.next().await.expect("No message").expect("Error reading message");
    let static_response: StaticResponse = from_value(from_str(&returned_message.to_string()).expect("Failed to convert response into JSON")).expect("Failed to convert JSON into StaticResponse");
    static_response.get_static_file().expect("No static file found")
}
