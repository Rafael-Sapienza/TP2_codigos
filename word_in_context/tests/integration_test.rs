use std::fs::File;
use std::io::{BufRead,BufReader};
use word_in_context::keyword_in_context;

#[test]
fn test_keyword_in_context() {
    keyword_in_context(
        "tests/input.txt".to_string(),
        "tests/stop_words.txt".to_string(),
    );
    let input_file = File::open("output.txt").expect("Erro ao ler arquivo de output");
    let reader = BufReader::new(&input_file);
    let mut lines = vec![String::new(), String::new()];
    for (i, line) in reader.lines().enumerate() {
        match line {
            Ok(line) => lines[i] = line,
            Err(_) => panic!("Erro ao ler linha"),
        }
    }
    let answer = vec!["b a".to_string(), "c d".to_string()];
    assert_eq!(lines, answer);
}
