use banter_shell as bsh;
use colored::*;
use std::io::{self, Write};

#[tokio::main]
async fn main() {
    // set path to .banterrc
    // TODO: add logging
    const BSH_PROFILE: &str = ".bsh_profile";
    let bsh_profile_path = bsh::set_home_dir_path(BSH_PROFILE);

    // parse command line arguments
    let matches = bsh::cli().get_matches();

    if matches.get_flag("message") {
        let mut ctx = if bsh_profile_path.exists() {
            // read .banterrc if it exists to get openai key and chat history
            let temp_ctx = bsh::read_context(&bsh_profile_path);
            // check if openai key is empty
            match temp_ctx.openai_key.is_empty() {
                // if openai key is empty, prompt user for openai key
                true => {
                    // get openai key from user
                    let openai_key = bsh::input(
                        "No OpenAI API key found, please enter:",
                        &mut io::stdin().lock(),
                        &mut io::stdout(),
                    );
                    bsh::new_context(&bsh_profile_path, openai_key.unwrap())
                }
                // else return context
                false => temp_ctx,
            }
        } else {
            // create .banterrc if it doesn't exist and prompt user for openai key
            let openai_key = bsh::input(
                "No OpenAI API key found, please enter:",
                &mut io::stdin().lock(),
                &mut io::stdout(),
            );
            bsh::new_context(&bsh_profile_path, openai_key.unwrap())
        };

        // call OpenAI API and display response
        let oai_response = bsh::call_oai(&ctx, &matches).await;
        match oai_response {
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
                let mut file = std::fs::File::create(&bsh_profile_path).unwrap();
                writeln!(file, "{}", ctx.openai_key).unwrap();
                for line in ctx.hist.iter().rev().take(6).rev() {
                    writeln!(file, "{}", line.replace('\n', "")).unwrap();
                }
            }
            Err(e) => println!("{}", e),
        }
    }

    // add
}
