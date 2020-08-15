#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
use line_bot_sdk_rust::{bot, model};
use rocket_contrib::json::Json;

#[post("/callback", format = "json", data = "<web_hook>")]
fn callback(web_hook: Json<model::WebHook>) -> &'static str {
  let bot_client = bot::LineBot::new(std::env::var("CHANNEL_TOKEN").unwrap());
  for event in web_hook.events.iter() {
    match event.r#type {
      "follow" | "join" => bot_client.reply_message(
        event,
        serde_json::json!([
        {
            "type": "text",
            "text": "follow, join event"
        }
        ]),
      ),
      "message" => {
        let message = event.message.as_ref().unwrap();
        match message.r#type {
          "text" => bot_client.reply_message(
            event,
            serde_json::json!([
            {
                "type": "text",
                "text": message.text.unwrap()
            }
            ]),
          ),
          _ => {}
        }
      }
      _ => {}
    }
  }
  "OK"
}

fn main() {
  rocket::ignite().mount("/", routes![callback]).launch();
}
