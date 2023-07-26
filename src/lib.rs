use clap::error::Result;
use clap::{Arg, ArgAction, ArgMatches, Command};
use dirs::home_dir;
use reqwest::{Client, StatusCode};
use serde_json::{json, Map, Value};
use std::io;
use std::io::Write;
use std::path::PathBuf;

pub fn set_home_dir_path(file_name: &str) -> PathBuf {
    let mut path = home_dir().unwrap();
    path.push(file_name);
    path
}

#[derive(PartialEq, Debug)]
pub struct Context {
    pub openai_key: String,
    pub hist: Vec<String>,
}

pub fn read_context(hist_file_path: &PathBuf) -> Context {
    // Initialize Context struct
    let mut ctx = Context {
        openai_key: String::from(""),
        hist: vec![],
    };
    // Read game file
    let saved = std::fs::read_to_string(hist_file_path).unwrap_or("".to_string());

    // if file is empty or doesn't exist, delete potential file return empty Context struct
    if saved.is_empty() {
        std::fs::remove_file(hist_file_path).unwrap();
        ctx
    } else {
        // get openai key from first line of file
        ctx.openai_key = saved.lines().next().unwrap().to_string();
        // get chat history from rest of file
        for line in saved.lines().skip(1) {
            ctx.hist.push(line.to_string());
        }
        ctx
    }
}

pub fn new_context(hist_file_path: &PathBuf, key: String) -> Context {
    // Initialize Context struct
    let ctx = Context {
        openai_key: key.trim().to_string(),
        hist: vec![],
    };
    // write openai key to file
    let mut file = std::fs::File::create(hist_file_path).unwrap();
    writeln!(file, "{}", ctx.openai_key).unwrap();

    ctx
}

pub fn input<R, W>(prompt: &str, mut reader: R, mut writer: W) -> Result<String, io::Error>
where
    R: io::BufRead,
    W: io::Write,
{
    match write!(writer, "{} ", prompt) {
        Ok(_) => {}
        Err(e) => return Err(e),
    }
    writer.flush()?;

    let mut input = String::new();
    reader.read_line(&mut input)?;
    let input = input.trim().to_lowercase();

    Ok(input)
}

pub fn cli() -> Command {
    Command::new("bsh")
        .about("A cli designed to facilitate seamless text-based conversations with ChatGPT.")
        .arg_required_else_help(true)
        .arg(
            Arg::new("message")
                .help("The message to send to ChatGPT in quotes.")
                .short('m')
                .long("message")
                .action(ArgAction::Set)
                .required(true),
        )
    // TODO: arg to remove local context
}

pub async fn call_oai(
    ctx: &Context,
    arg: &ArgMatches,
) -> Result<Value, Box<dyn std::error::Error>> {
    let new_msg = arg.get_one::<String>("message").unwrap();

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
    let client: Client = reqwest::Client::new();
    let resp = client
        .post(url)
        .header("Content-Type", "application/json")
        .header(
            "Authorization",
            "Bearer ".to_owned() + ctx.openai_key.as_str(),
        )
        .json(&body_json)
        .send()
        .await?;

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
