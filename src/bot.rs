use crate::model;
use base64::encode;
use hmac::{Hmac, Mac, NewMac};
use sha2::Sha256;

pub struct LineBot {
    pub channel_token: String,
    pub channel_secret: String,
}

impl LineBot {
    pub fn new(channel_token: String, channel_secret: String) -> LineBot {
        LineBot {
            channel_token: channel_token,
            channel_secret: channel_secret,
        }
    }

    pub fn check_signature(&self, body: &str, signature: &str) -> bool {
        let mut mac = Hmac::<Sha256>::new_varkey(self.channel_secret.as_bytes()).unwrap();
        mac.update(body.as_bytes());
        signature == encode(&mac.finalize().into_bytes().to_vec())
    }

    pub fn reply_message(&self, event: &model::Event, messages: serde_json::Value) {
        let res = reqwest::blocking::Client::default()
            .post("https://api.line.me/v2/bot/message/reply")
            .header("User-Agent", "line-bot-sdk-rust/0.1.0")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.channel_token))
            .json(&serde_json::json!({
                "replyToken": event.reply_token,
                "messages": messages
            }))
            .send();
        match res {
            Ok(res) => println!(
                "POST https://api.line.me/v2/bot/message/reply {:} {:}",
                res.status(),
                res.text().unwrap()
            ),
            Err(err) => println!(
                "POST https://api.line.me/v2/bot/message/reply error {}",
                err
            ),
        }
    }
}
