use word_in_context::keyword_in_context;

fn main() {
	keyword_in_context( "../input_exemplo_do_teams/input.txt".to_string(),
        "../input_exemplo_do_teams/stop_words.txt".to_string()
	);
}
