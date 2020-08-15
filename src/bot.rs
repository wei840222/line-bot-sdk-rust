use crate::model;

pub struct LineBot {
    pub channel_token: String,
}

impl LineBot {
    pub fn new(channel_token: String) -> LineBot {
        LineBot {
            channel_token: channel_token,
        }
    }

    pub fn reply_message(&self, event: &model::Event, messages: serde_json::Value) {
        let res = reqwest::blocking::Client::default()
            .post("https://api.line.me/v2/bot/message/reply")
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
