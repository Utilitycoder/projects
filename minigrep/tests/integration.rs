use minigrep::Config;

#[test]
fn test_minigrep() {
    let config = Config::build(
        vec![
            String::from("minigrep"),
            String::from("to"),
            String::from("poem.txt"),
        ].into_iter()
    ).unwrap();

    assert_eq!(config.query, "to");

    let result = minigrep::run(config);
    assert!(result.is_ok());
}

