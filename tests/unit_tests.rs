#[test]
fn find_a_match() {
    let mut result = Vec::new();

    grrs::find_matches("cat\n", "cat", &mut result);
    assert_eq!(result, b"cat\n\n");
}
