use std::env;
use word_in_context::keyword_in_context;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path: String = if args.len() > 1 {
        args[1].clone()
    } else {
        "../input_exemplo_do_teams/input.txt".to_string()
    };
    let stop_words_path: String = if args.len() > 2 {
        args[2].clone()
    } else {
        "../input_exemplo_do_teams/stop_words.txt".to_string()
    };
    keyword_in_context(input_path, stop_words_path);
}
