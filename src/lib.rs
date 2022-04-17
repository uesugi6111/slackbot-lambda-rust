use chrono::{TimeZone, Utc};
use reqwest::{
    header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE},
    Response,
};
use serde::Serialize;
use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, Serialize)]
pub struct Output {
    pub message: String,
}

pub async fn run() -> Result<Output, Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let message = generate_message();
    let body = generate_post_body(&dotenv::var("channel").unwrap(), &message);
    let resp = post_message(body, &dotenv::var("token").unwrap()).await?;
    println!("{:#?}", resp);
    Ok(Output {
        message: resp
            .text()
            .await
            .unwrap_or_else(|_| "response convert error".to_string()),
    })
}

fn generate_message() -> String {
    let tz = chrono_tz::Asia::Tokyo;
    let now = Utc::now().with_timezone(&tz);
    let nokori = (Utc.ymd(2022, 10, 9).with_timezone(&tz) - now.date()).num_days();

    match nokori.cmp(&0) {
        Ordering::Less => "試験日が過ぎています。終わりました。".to_owned(),
        Ordering::Equal => "試験日当日です。".to_owned(),
        Ordering::Greater => format!(
            "情報処理安全確保支援士試験試験まで{}日です。勉強してください。",
            nokori
        ),
    }
}

async fn post_message(
    body: HashMap<String, String>,
    token: &str,
) -> Result<Response, Box<dyn std::error::Error>> {
    dbg!(&body);
    let client = reqwest::Client::new();
    let resp = client
        .post("https://slack.com/api/chat.postMessage")
        .headers(generate_header(token))
        .json(&body)
        .send()
        .await?;
    Ok(resp)
}

fn generate_post_body(channel: &str, message: &str) -> HashMap<String, String> {
    let mut body = HashMap::new();
    body.insert("channel".to_string(), "#".to_owned() + channel);
    body.insert("text".to_string(), message.to_string());
    body
}

fn generate_header(token: &str) -> HeaderMap {
    let mut header = HeaderMap::new();
    header.insert(AUTHORIZATION, format!("Bearer {}", token).parse().unwrap());
    header.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    header
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aa() {
        let tz = chrono_tz::Asia::Tokyo;
        let datetime = Utc::now().with_timezone(&tz);
        let aaa = Utc.ymd(2021, 6, 13).with_timezone(&tz);
        dbg!(datetime);
        dbg!(aaa);
        dbg!((aaa - datetime.date()).num_days());
    }
    #[test]
    fn aaa() {
        dbg!(generate_message());
    }
    #[tokio::test]
    async fn run_test() {
        dbg!(run().await.unwrap());
    }

    use dotenv::dotenv;
    use std::env;
    #[test]
    fn main_m() {
        dotenv().ok();

        for (key, value) in env::vars() {
            println!("{}: {}", key, value);
        }
    }
}
