#[test]
fn simplify_test() {
    let text = "'hel\"lo' \"hel'lo\" 'hel\\'lo' \"hel\\\"lo\"".to_string();
    let (s, q) = super::simplify(text).unwrap();
    assert_eq!(s, "\"0\" \"1\" \"2\" \"3\"");
    assert_eq!(q, vec!["hel\"lo", "hel'lo", "hel\\'lo", "hel\\\"lo"]);
}
