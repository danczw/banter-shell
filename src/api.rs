use clap::ArgMatches;
use reqwest::{header, Client, StatusCode};
use serde_json::{json, Map, Value};
use std::time::Duration;

use crate::context;

pub async fn call_oai(
    ctx: &context::Context,
    arg: &ArgMatches,
) -> Result<Value, Box<dyn std::error::Error>> {
    let new_msg = arg.get_one::<String>("message").unwrap();

    // Build the headers
    let mut headers = header::HeaderMap::new();
    headers.insert(
        "Content-Type",
        header::HeaderValue::from_static("application/json"),
    );
    let auth_value = format!("Bearer {}", ctx.openai_key.as_str());
    let mut auth_value = header::HeaderValue::from_str(&auth_value).unwrap();
    auth_value.set_sensitive(true);
    headers.insert(header::AUTHORIZATION, auth_value);

    // Build the URL
    let url = "https://api.openai.com/v1/chat/completions";

    // Build the body
    let mut body: Map<String, Value> = Map::new();
    body.insert("model".to_string(), json!("gpt-4"));

    let mut messages = Vec::new();
    for ctx_msg in ctx.hist.iter() {
        let role = ctx_msg.split("||").next().unwrap();
        let content = ctx_msg.split("||").nth(1).unwrap();
        messages.push(json!({"role": role, "content": content}));
    }
    messages.push(json!({"role": "user", "content": new_msg}));
    body.insert("messages".to_string(), Value::Array(messages));
    let body_json = Value::Object(body);

    // Initialize client and send request
    let client = Client::builder()
        .default_headers(headers)
        .timeout(Duration::new(120, 0))
        .build()?;
    let resp = client.post(url).json(&body_json).send().await?;

    check_response(resp).await
}

pub async fn check_response(resp: reqwest::Response) -> Result<Value, Box<dyn std::error::Error>> {
    // Get response values
    let resp_status = resp.status();
    // Deserialize response text
    let resp_text = resp.text().await?;
    let resp_json: Value = serde_json::from_str(&resp_text)?;

    // check response
    match resp_status {
        StatusCode::OK => {
            // return response text
            Ok(resp_json)
        }
        _ => {
            // return error message
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                resp_text,
            )))
        }
    }
}
