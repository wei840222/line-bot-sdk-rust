#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
use line_bot_sdk_rust::{bot, model};
use rocket::data::{self, FromDataSimple};
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Data;
use rocket::Outcome::{Failure, Success};
use std::io::Read;

pub struct Signature {
  pub key: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for Signature {
  type Error = ();
  fn from_request(request: &'a Request<'r>) -> request::Outcome<Signature, ()> {
    let keys: Vec<_> = request.headers().get("X-Line-Signature").collect();
    if keys.len() != 1 {
      return Failure((Status::BadRequest, ()));
    }
    Success(Signature {
      key: keys[0].to_string(),
    })
  }
}

pub struct Body {
  pub data: String,
}

impl FromDataSimple for Body {
  type Error = String;
  fn from_data(_: &Request, data: Data) -> data::Outcome<Self, String> {
    let mut string = String::new();
    if let Err(e) = data.open().read_to_string(&mut string) {
      return Failure((Status::InternalServerError, format!("{:?}", e)));
    }
    Success(Body { data: string })
  }
}

#[post("/callback", format = "json", data = "<body>")]
fn callback(signature: Signature, body: Body) -> Status {
  let bot_client = bot::LineBot::new(
    std::env::var("CHANNEL_TOKEN").unwrap(),
    std::env::var("CHANNEL_SECRET").unwrap(),
  );
  if !bot_client.check_signature(&body.data, &signature.key) {
    return Status::BadRequest;
  }

  let web_hook: model::WebHook = serde_json::from_str(&body.data).unwrap();
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
  Status::Ok
}

fn main() {
  rocket::ignite().mount("/", routes![callback]).launch();
}
