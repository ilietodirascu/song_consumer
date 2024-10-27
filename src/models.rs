use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct RabbitMessage {
    pub chat_id: i64,
    pub text: String,
}

#[derive(Deserialize, Debug)]
pub struct YouTubeResponse {
    pub items: Vec<YouTubeItem>,
}

#[derive(Deserialize, Debug)]
pub struct YouTubeItem {
    pub id: YouTubeVideoId,
    pub snippet: Snippet, // Add snippet for title filtering
}

#[derive(Deserialize, Debug)]
pub struct YouTubeVideoId {
    pub videoId: String,
}

#[derive(Deserialize, Debug)]
pub struct Snippet {
    pub title: String, // Video title to improve filtering
}

#[derive(Deserialize, Debug)]
pub struct Tomp3Response {
    pub links: Option<Links>,
}

#[derive(Deserialize, Debug)]
pub struct Links {
    pub mp3: Option<HashMap<String, Mp3Link>>,
}

#[derive(Deserialize, Debug)]
pub struct Mp3Link {
    pub k: String,
}

#[derive(Deserialize, Debug)]
pub struct ConvertResponse {
    pub dlink: String, // Direct download link
}
