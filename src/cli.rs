use clap::{Arg, ArgAction, Command};

pub fn cli() -> Command {
    Command::new("gtc")
        .about("A cli designed to facilitate seamless text-based conversations with ChatGPT.")
        .arg_required_else_help(true)
        .arg(
            Arg::new("message")
                .help("The message to send to ChatGPT in quotes.")
                // .short('m')
                // .long("message")
                .index(1)
                .action(ArgAction::Set)
                .required(true),
        )
    // TODO: arg to remove local context
    // TODO: arg to show version
}
