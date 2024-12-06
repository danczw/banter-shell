#[cfg(test)]
mod tests {
    use dirs::home_dir;
    use gtc::{api, context, helper};
    use std::fs::File;
    use std::io::{self, Write};
    use std::path::PathBuf;

    // set_home_dir_path tests
    #[test]
    fn test_set_home_dir_path() {
        let file_name = "test.txt";
        let expected_path = home_dir().unwrap().join(file_name);
        assert_eq!(helper::set_home_dir_path(file_name), expected_path);
    }

    #[test]
    fn test_set_home_dir_path_with_subdir() {
        let file_name = "test.txt";
        let subdir = "subdir";
        let expected_path = home_dir().unwrap().join(subdir).join(file_name);
        assert_eq!(
            helper::set_home_dir_path(&format!("{}/{}", subdir, file_name)),
            expected_path
        );
    }

    // read_context tests
    #[test]
    fn test_read_context() {
        let mut file = File::create(".test_read_context").unwrap();
        writeln!(file, "openai_key").unwrap();
        writeln!(file, "message 1").unwrap();
        writeln!(file, "message 2").unwrap();
        file.flush().unwrap();

        let context_file_path = PathBuf::from(".test_read_context");
        let expected_context = context::Context {
            openai_key: "openai_key".to_string(),
            hist: vec!["message 1".to_string(), "message 2".to_string()],
        };
        assert_eq!(context::read_context(&context_file_path), expected_context);

        std::fs::remove_file(".test_read_context").unwrap();
    }

    #[test]
    fn test_read_context_with_empty_file() {
        let mut file = File::create(".test_read_context_with_empty_file").unwrap();
        file.flush().unwrap();

        let context_file_path = PathBuf::from(".test_read_context_with_empty_file");
        let expected_context = context::Context {
            openai_key: "".to_string(),
            hist: vec![],
        };
        assert_eq!(context::read_context(&context_file_path), expected_context);
    }

    #[test]
    #[should_panic(expected = "No such file or directory")]
    fn test_read_context_with_invalid_file() {
        let context_file_path = PathBuf::from("/invalid/path");
        context::read_context(&context_file_path);
    }

    #[test]
    fn test_read_context_with_empty_key() {
        let mut file = File::create(".test_read_context_with_empty_key").unwrap();
        writeln!(file).unwrap();
        writeln!(file, "message 1").unwrap();
        writeln!(file, "message 2").unwrap();
        file.flush().unwrap();

        let context_file_path = PathBuf::from(".test_read_context_with_empty_key");
        let expected_context = context::Context {
            openai_key: "".to_string(),
            hist: vec!["message 1".to_string(), "message 2".to_string()],
        };
        assert_eq!(context::read_context(&context_file_path), expected_context);

        std::fs::remove_file(".test_read_context_with_empty_key").unwrap();
    }

    #[test]
    fn test_read_context_with_empty_history() {
        let mut file = File::create(".test_read_context_with_empty_history").unwrap();
        writeln!(file, "openai_key").unwrap();
        file.flush().unwrap();

        let context_file_path = PathBuf::from(".test_read_context_with_empty_history");
        let expected_context = context::Context {
            openai_key: "openai_key".to_string(),
            hist: vec![],
        };
        assert_eq!(context::read_context(&context_file_path), expected_context);

        std::fs::remove_file(".test_read_context_with_empty_history").unwrap();
    }

    // input tests
    #[test]
    fn test_input() {
        let mut writer = Vec::new();
        let reader = io::Cursor::new(b"yes");
        let result = helper::input("Does this test pass?", reader, &mut writer).unwrap();
        assert_eq!(result, "yes");
        assert_eq!(writer, b"Does this test pass? ");
    }

    // TODO: test call_oai

    // test check_response
    #[tokio::test]
    async fn test_check_response_ok() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();

        // create mock response with status code 200 OK and some JSON data
        let _mock = server
            .mock("POST", "/")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"foo": "bar"}"#)
            .create();

        // create new reqwest client and send request to mock server
        let client = reqwest::Client::new();
        let resp = client
            .post(&url)
            .body(r#"{"foo": "bar"}"#)
            .send()
            .await
            .unwrap();

        let result = api::check_response(resp).await;
        assert_eq!(result.unwrap(), serde_json::json!({"foo": "bar"}));
        assert!(true)
    }

    #[tokio::test]
    async fn test_check_response_err() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();

        // create mock response with status code 200 OK and some JSON data
        let _mock = server
            .mock("POST", "/")
            .with_status(400)
            .with_header("content-type", "application/json")
            .with_body(r#"{"error": "bad request"}"#)
            .create();

        // create new reqwest client and send request to mock server
        let client = reqwest::Client::new();
        let resp = client
            .post(&url)
            .body(r#"{"foo": "bar"}"#)
            .send()
            .await
            .unwrap();

        let result = api::check_response(resp).await;
        assert_eq!(
            result.unwrap_err().to_string(),
            r#"{"error": "bad request"}"#
        );

        assert!(true)
    }
}
