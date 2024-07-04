use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum ParseMode {
    HTML,
    MarkdownV2,
}

impl std::fmt::Display for ParseMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    ok: bool,
    result: Option<MessageResult>,
    error_code: Option<u16>,
    description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageResult {
    message_id: u32,
    date: u64,
}

pub struct Bot {
    base_url: String,
}

impl Bot {
    pub fn new(token: String) -> Self {
        Self {
            base_url: format!("https://api.telegram.org/bot{}/", token),
        }
    }

    pub fn send_message(
        self,
        chat_id: i64,
        text: String,
        parse_mode: Option<ParseMode>,
    ) -> Result<Response, reqwest::Error> {
        let command: &str = "sendMessage";
        let mut final_parse_mode = ParseMode::HTML;
        if parse_mode.is_some() {
            final_parse_mode = parse_mode.unwrap();
        }
        let res: Result<reqwest::blocking::Response, reqwest::Error> =
            reqwest::blocking::get(format!(
                "{}{}?chat_id={}&text={}&parse_mode={}",
                self.base_url,
                command,
                chat_id,
                text,
                final_parse_mode.to_string()
            ));

        match res {
            Ok(response) => {
                let text: String = response.text().unwrap();
                let data: Response = serde_json::from_str(text.as_str()).unwrap();
                Ok(data)
            }
            Err(error) => Err(error),
        }
    }
}

fn main() {
    let token = "<bot_token>"; // Replace it with bot token
    let bot: Bot = Bot::new(token.to_owned());

    let chat_id: i64 = 0; // Replace it with chat id
    let parse_mode: Option<ParseMode> = Some(ParseMode::HTML);

    // Your custom message
    let text: String = "Hi, This is an example test message".to_owned();

    let result: Result<Response, reqwest::Error> = bot.send_message(chat_id, text, parse_mode);
    match result {
        Ok(response) => println!("{:?}", response),
        Err(error) => println!("{:?}", error),
    }
}
