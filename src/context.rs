use std::path::PathBuf;

#[derive(PartialEq, Debug)]
pub struct Context {
    pub openai_key: String,
    pub hist: Vec<String>,
}

pub fn read_context(hist_file_path: &PathBuf) -> Context {
    let mut ctx = Context {
        openai_key: String::from(""),
        hist: vec![],
    };
    // read profile file
    let saved = std::fs::read_to_string(hist_file_path).unwrap_or("".to_string());

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
