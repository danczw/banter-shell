use colored::*;
use log::{debug, error, info}; // trace, warn
use std::io::{self, Write};

mod api;
mod cli;
mod context;
mod helper;
mod logger;

#[tokio::main]
async fn main() {
    let log_path = helper::set_home_dir_path(".gtc.log");
    let log_config = logger::setup_logger(log_path);
    let _handle = log4rs::init_config(log_config).unwrap();

    const PROFILE: &str = ".gtc";
    let profile_path = helper::set_home_dir_path(PROFILE);
    debug!("profile_path - {}", profile_path.to_string_lossy());

    let matches = cli::cli().get_matches();

    if matches.contains_id("message") {
        let mut ctx = if profile_path.exists() {
            let mut ctx_read = context::read_context(&profile_path);

            match ctx_read.openai_key.is_empty() {
                true => {
                    info!("No OpenAI API key found");
                    let openai_key = helper::input(
                        "No OpenAI API key found, please enter:",
                        &mut io::stdin().lock(),
                        &mut io::stdout(),
                    );
                    // update context and return
                    ctx_read.openai_key = openai_key.unwrap().trim().to_string();
                    ctx_read.hist = vec![];
                    ctx_read
                }
                false => ctx_read,
            }
        } else {
            // create profile if not existing and prompt for openai key
            let openai_key = helper::input(
                "No OpenAI API key found, please enter:",
                &mut io::stdin().lock(),
                &mut io::stdout(),
            );
            // update context
            context::Context {
                openai_key: openai_key.unwrap().trim().to_string(),
                hist: vec![],
            }
        };

        let oai_response = api::call_oai(&ctx, &matches).await.unwrap();
        let check_response = api::check_response(oai_response).await;
        match check_response {
            Ok(resp_value) => {
                let answer = resp_value["choices"][0]["message"]["content"]
                    .as_str()
                    .unwrap();
                println!("{}", answer.cyan());
                // add message and answer to chat history
                ctx.hist.push(
                    "user||".to_owned() + matches.get_one::<String>("message").unwrap().as_str(),
                );
                ctx.hist.push("assistant||".to_owned() + answer);

                // clear profile file and write key as well as last 6 messages to file
                let mut file = std::fs::File::create(&profile_path).unwrap();
                writeln!(file, "{}", ctx.openai_key).unwrap();
                for line in ctx.hist.iter().rev().take(6).rev() {
                    writeln!(file, "{}", line.replace('\n', "")).unwrap();
                }
            }
            Err(e) => {
                error!("OAI response error - {}", e);
                println!("Bernard, we have a problem!");
            }
        }
    }
}
