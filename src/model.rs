use serde::Deserialize;

#[derive(Deserialize)]
pub struct WebHook<'a> {
    pub destination: &'a str,
    pub events: Vec<Event<'a>>,
}

#[derive(Deserialize)]
pub struct Event<'a> {
    #[serde(rename(deserialize = "replyToken"))]
    pub reply_token: Option<&'a str>,
    pub r#type: &'a str,
    pub mode: &'a str,
    pub timestamp: i64,
    pub source: Source<'a>,
    pub message: Option<Message<'a>>,
}

#[derive(Deserialize)]
pub struct Source<'a> {
    pub r#type: &'a str,
    #[serde(rename(deserialize = "groupId"))]
    pub group_id: Option<&'a str>,
    #[serde(rename(deserialize = "roomId"))]
    pub room_id: Option<&'a str>,
    #[serde(rename(deserialize = "userId"))]
    pub user_id: &'a str,
}

#[derive(Deserialize)]
pub struct Message<'a> {
    pub id: &'a str,
    pub r#type: &'a str,
    pub text: Option<&'a str>,
}
