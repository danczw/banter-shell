use dirs::home_dir;
use std::io;
use std::path::PathBuf;

pub fn set_home_dir_path(file_name: &str) -> PathBuf {
    let mut path = home_dir().unwrap();
    path.push(file_name);
    path
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
    let input = input.trim();

    Ok(input.to_string())
}
