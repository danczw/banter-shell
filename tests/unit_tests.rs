#[cfg(test)]
mod tests {
    use dirs::home_dir;
    use gtc;
    use std::fs::File;
    use std::io::{self, Write};
    use std::path::PathBuf;

    // set_home_dir_path tests
    #[test]
    fn test_set_home_dir_path() {
        let file_name = "test.txt";
        let expected_path = home_dir().unwrap().join(file_name);
        assert_eq!(gtc::set_home_dir_path(file_name), expected_path);
    }

    #[test]
    fn test_set_home_dir_path_with_subdir() {
        let file_name = "test.txt";
        let subdir = "subdir";
        let expected_path = home_dir().unwrap().join(subdir).join(file_name);
        assert_eq!(
            gtc::set_home_dir_path(&format!("{}/{}", subdir, file_name)),
            expected_path
        );
    }

    // read_context tests
    #[test]
    fn test_read_context() {
        let mut file = File::create(".gtc_profile").unwrap();
        writeln!(file, "openai_key").unwrap();
        writeln!(file, "message 1").unwrap();
        writeln!(file, "message 2").unwrap();
        file.flush().unwrap();

        let context_file_path = PathBuf::from(".gtc_profile");
        let expected_context = gtc::Context {
            openai_key: "openai_key".to_string(),
            hist: vec!["message 1".to_string(), "message 2".to_string()],
        };
        assert_eq!(gtc::read_context(&context_file_path), expected_context);

        std::fs::remove_file(".gtc_profile").unwrap();
    }

    #[test]
    fn test_read_context_with_empty_file() {
        let mut file = File::create(".gtc_profile").unwrap();
        file.flush().unwrap();

        let context_file_path = PathBuf::from(".gtc_profile");
        let expected_context = gtc::Context {
            openai_key: "".to_string(),
            hist: vec![],
        };
        assert_eq!(gtc::read_context(&context_file_path), expected_context);
    }

    #[test]
    #[should_panic(expected = "No such file or directory")]
    fn test_read_context_with_invalid_file() {
        let context_file_path = PathBuf::from("/invalid/path");
        gtc::read_context(&context_file_path);
    }

    #[test]
    fn test_read_context_with_empty_key() {
        let mut file = File::create(".gtc_profile").unwrap();
        writeln!(file).unwrap();
        writeln!(file, "message 1").unwrap();
        writeln!(file, "message 2").unwrap();
        file.flush().unwrap();

        let context_file_path = PathBuf::from(".gtc_profile");
        let expected_context = gtc::Context {
            openai_key: "".to_string(),
            hist: vec!["message 1".to_string(), "message 2".to_string()],
        };
        assert_eq!(gtc::read_context(&context_file_path), expected_context);

        std::fs::remove_file(".gtc_profile").unwrap();
    }

    #[test]
    fn test_read_context_with_empty_history() {
        let mut file = File::create(".gtc_profile").unwrap();
        writeln!(file, "openai_key").unwrap();
        file.flush().unwrap();

        let context_file_path = PathBuf::from(".gtc_profile");
        let expected_context = gtc::Context {
            openai_key: "openai_key".to_string(),
            hist: vec![],
        };
        assert_eq!(gtc::read_context(&context_file_path), expected_context);

        std::fs::remove_file(".gtc_profile").unwrap();
    }

    // input tests
    #[test]
    fn test_input() {
        let mut writer = Vec::new();
        let reader = io::Cursor::new(b"yes");
        let result = gtc::input("Does this test pass?", reader, &mut writer).unwrap();
        assert_eq!(result, "yes");
        assert_eq!(writer, b"Does this test pass? ");
    }
    // TODO: test call_oai

    // TODO: test check_response
}
