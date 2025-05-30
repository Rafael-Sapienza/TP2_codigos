use std::cmp::Ordering;
use std::cmp::min;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
//use std::fs;
//use std::path::Path;
//use std::collections::HashMap;

pub fn keyword_in_context() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!(
            "Please provide the input path and the stop words path as command-line arguments."
        );
        return;
    }
    let input_path: String = args[1].clone();
    let stop_words_path: String = args[2].clone();
    input_file_to_vector(stop_words_file_to_set, input_path, stop_words_path);
}

fn input_file_to_vector(
    func: fn(
        fn(
            fn(
                fn(
                    fn(fn(), Vec<Vec<String>>) -> Vec<Vec<String>>,
                    Vec<Vec<String>>,
                ) -> Vec<Vec<String>>,
                Vec<Vec<String>>,
                Vec<Vec<u64>>,
            ) -> Vec<Vec<String>>,
            &Vec<Vec<String>>,
            &HashSet<String>,
        ) -> Vec<Vec<u64>>,
        String,
        Vec<Vec<String>>,
    ) -> HashSet<String>,
    input_txt: String,
    stop_words_txt: String,
) -> Vec<Vec<String>> {
    let mut input_vector: Vec<Vec<String>> = Vec::new();
    let input_file = File::open(&input_txt);
    match input_file {
        Ok(input_file) => {
            let reader = BufReader::new(input_file);
            for line in reader.lines() {
                match line {
                    Ok(line) => {
                        input_vector.push(Vec::new());
                        let mut word: String = String::new();
                        if let Some(current_line) = input_vector.last_mut() {
                            for caractere in line.chars() {
                                if caractere.is_alphanumeric() {
                                    word.push(caractere.clone());
                                } else if !(word.is_empty()) {
                                    current_line.push(word.clone());
                                    word.clear();
                                }
                            }
                            if !word.is_empty() {
                                current_line.push(word);
                            }
                        }
                    }
                    Err(_) => panic!("Erro ao ler linha"),
                }
            }
        }
        Err(e) => panic!("Erro ao ler o arquivo '{}' : {}", input_txt, e),
    }
    func(find_key_words_indexes, stop_words_txt, input_vector.clone());
    input_vector
}

fn stop_words_file_to_set(
    func: fn(
        fn(
            fn(fn(fn(), Vec<Vec<String>>) -> Vec<Vec<String>>, Vec<Vec<String>>) -> Vec<Vec<String>>,
            Vec<Vec<String>>,
            Vec<Vec<u64>>,
        ) -> Vec<Vec<String>>,
        &Vec<Vec<String>>,
        &HashSet<String>,
    ) -> Vec<Vec<u64>>,
    stop_words_txt: String,
    input_vector: Vec<Vec<String>>,
) -> HashSet<String> {
    let mut stop_words_set: HashSet<String> = HashSet::new();
    let stop_words_file = File::open(&stop_words_txt);
    match stop_words_file {
        Ok(stop_words_file) => {
            let reader = BufReader::new(stop_words_file);
            for line in reader.lines() {
                match line {
                    Ok(word) => {
                        let single_stop_word: String = word.trim().to_string();
                        stop_words_set.insert(single_stop_word);
                    }
                    Err(e) => {
                        panic!("Erro ao ler o arquivo '{}': {}", stop_words_txt, e);
                    }
                }
            }
        }
        Err(e) => {
            panic!("Erro ao ler o arquivo '{}': {}", stop_words_txt, e);
        }
    }
    func(
        generate_circularly_shifted_lists,
        &input_vector,
        &stop_words_set,
    );
    stop_words_set
}

fn find_key_words_indexes(
    func: fn(
        fn(fn(fn(), Vec<Vec<String>>) -> Vec<Vec<String>>, Vec<Vec<String>>) -> Vec<Vec<String>>,
        Vec<Vec<String>>,
        Vec<Vec<u64>>,
    ) -> Vec<Vec<String>>,
    input_vector: &Vec<Vec<String>>,
    stop_words_set: &HashSet<String>,
) -> Vec<Vec<u64>> {
    let mut key_words_occurrences: Vec<Vec<u64>> = vec![Vec::new(); input_vector.len()];
    for (i, line) in input_vector.iter().enumerate() {
        for (j, string) in line.iter().enumerate() {
            let string_aux = string.to_lowercase();
            if !stop_words_set.contains(&string_aux) {
                key_words_occurrences[i].push(j as u64)
            }
        }
    }
    func(
        sort_circularly_shifted_lists_alphabetically,
        input_vector.to_vec(),
        key_words_occurrences.clone(),
    );
    key_words_occurrences
}

fn generate_circularly_shifted_lists(
    func: fn(fn(fn(), Vec<Vec<String>>) -> Vec<Vec<String>>, Vec<Vec<String>>) -> Vec<Vec<String>>,
    input_vector: Vec<Vec<String>>,
    key_words_occurrences: Vec<Vec<u64>>,
) -> Vec<Vec<String>> {
    let mut circularly_shifted_lists: Vec<Vec<String>> = Vec::new();
    for (i, line) in input_vector.iter().enumerate() {
        for j in &key_words_occurrences[i] {
            let j = *j as usize;
            {
                let before = &line[..j];
                let after = &line[j..];
                let circular_shifted_list = after.iter().chain(before.iter()).cloned().collect();
                circularly_shifted_lists.push(circular_shifted_list);
            }
        }
    }
    func(print_final_list, circularly_shifted_lists.clone());
    circularly_shifted_lists
}

fn sort_circularly_shifted_lists_alphabetically(
    func: fn(fn(), Vec<Vec<String>>) -> Vec<Vec<String>>,
    mut circularly_shifted_lists: Vec<Vec<String>>,
) -> Vec<Vec<String>> {
    fn compare_alphabetically(a: &Vec<String>, b: &Vec<String>) -> Ordering {
        if a.is_empty() || a[0].is_empty() {
            return Ordering::Less;
        }
        if b.is_empty() || b[0].is_empty() {
            return Ordering::Greater;
        }
        for i in 0..min(a.len(), b.len()) {
            let first_char = a[i].chars().next().unwrap().to_lowercase().next().unwrap();
            let second_char = b[i].chars().next().unwrap().to_lowercase().next().unwrap();
            let order: Ordering = first_char.cmp(&second_char);
            if order != Ordering::Equal {
                return order;
            }
        }
        //Desempata pelo comprimento de vetores (numero de palavras no titulo)
        a.len().cmp(&b.len())
    }
    circularly_shifted_lists.sort_by(|a, b| compare_alphabetically(a, b));
    func(no_op, circularly_shifted_lists.clone());
    circularly_shifted_lists
}

fn print_final_list(func: fn(), final_list: Vec<Vec<String>>) -> Vec<Vec<String>> {
    println!("{:?}", final_list);
    func();
    final_list
}

fn no_op() {}

#[cfg(test)]
mod tests {
    use super::*;

    fn ghost_stop_words_file_to_set(
        _func: fn(
            fn(
                fn(
                    fn(fn(), Vec<Vec<String>>) -> Vec<Vec<String>>,
                    Vec<Vec<String>>,
                ) -> Vec<Vec<String>>,
                Vec<Vec<String>>,
                Vec<Vec<u64>>,
            ) -> Vec<Vec<String>>,
            &Vec<Vec<String>>,
            &HashSet<String>,
        ) -> Vec<Vec<u64>>,
        _stop_words_txt: String,
        _input_vector: Vec<Vec<String>>,
    ) -> HashSet<String> {
        HashSet::new()
    }

    fn ghost_find_key_words_indexes(
        _func: fn(
            fn(fn(fn(), Vec<Vec<String>>) -> Vec<Vec<String>>, Vec<Vec<String>>) -> Vec<Vec<String>>,
            Vec<Vec<String>>,
            Vec<Vec<u64>>,
        ) -> Vec<Vec<String>>,
        _input_vector: &Vec<Vec<String>>,
        _stop_words_set: &HashSet<String>,
    ) -> Vec<Vec<u64>> {
        Vec::new()
    }

    fn ghost_generate_circularly_shifted_lists(
        _func: fn(
            fn(fn(), Vec<Vec<String>>) -> Vec<Vec<String>>,
            Vec<Vec<String>>,
        ) -> Vec<Vec<String>>,
        _input_vector: Vec<Vec<String>>,
        _key_words_occurrences: Vec<Vec<u64>>,
    ) -> Vec<Vec<String>> {
        Vec::new()
    }

    fn ghost_sort_circularly_shifted_lists_alphabetically(
        _func: fn(fn(), Vec<Vec<String>>) -> Vec<Vec<String>>,
        mut _circularly_shifted_lists: Vec<Vec<String>>,
    ) -> Vec<Vec<String>> {
        Vec::new()
    }

    fn ghost_print_final_list(_func: fn(), _final_list: Vec<Vec<String>>) -> Vec<Vec<String>> {
        Vec::new()
    }

    #[test]
    fn test_input_file_to_vector() {
        let resultado_real = input_file_to_vector(
            ghost_stop_words_file_to_set,
            "../inputs_para_teste/input_para_teste_1.txt".to_string(),
            "".to_string(),
        );
        let resultado_esperado: Vec<Vec<String>> = vec![
            vec![
                "Understanding".to_string(),
                "Unit".to_string(),
                "and".to_string(),
                "Integration".to_string(),
                "Testing".to_string(),
                "in".to_string(),
                "Rust".to_string(),
            ],
            vec![
                "A".to_string(),
                "Beginner".to_string(),
                "s".to_string(),
                "Guide".to_string(),
                "to".to_string(),
                "Testing".to_string(),
                "in".to_string(),
                "Rust".to_string(),
            ],
        ];
        assert_eq!(resultado_real, resultado_esperado);
    }

    #[test]
    fn test_stop_words_file_to_set() {
        let output = stop_words_file_to_set(
            ghost_find_key_words_indexes,
            "../inputs_para_teste/stop_words_para_teste_1.txt".to_string(),
            vec![],
        );
        let mut answer: HashSet<String> = HashSet::new();
        let sw = vec!["and".to_string(), "or".to_string(), "not".to_string()];
        for s in sw {
            answer.insert(s);
        }
        assert_eq!(answer, output);
    }

    #[test]
    fn test_find_key_words_indexes() {
        let input = vec![
            vec!["a".to_string(), "b".to_string()],
            vec!["c".to_string(), "d".to_string()],
        ];
        let mut sw: HashSet<String> = HashSet::new();
        sw.insert("b".to_string());
        sw.insert("c".to_string());
        let output = find_key_words_indexes(ghost_generate_circularly_shifted_lists, &input, &sw);
        let answer = vec![vec![0], vec![1]];
        assert_eq!(output, answer);
    }

    #[test]
    fn test_generate_circularly_shifted_lists() {
        let input = vec![
            vec!["a".to_string(), "b".to_string()],
            vec!["c".to_string(), "d".to_string()],
        ];
        let occs = vec![vec![0], vec![1]];
        let output = generate_circularly_shifted_lists(
            ghost_sort_circularly_shifted_lists_alphabetically,
            input,
            occs,
        );
        let answer = vec![
            vec!["a".to_string(), "b".to_string()],
            vec!["d".to_string(), "c".to_string()],
        ];
        assert_eq!(output, answer);
    }

    #[test]
    fn test_sort_circularly_shifted_lists_alphabetically() {
        let input = vec![
            vec!["c".to_string(), "d".to_string()],
            vec!["a".to_string(), "b".to_string()],
        ];
        let output = sort_circularly_shifted_lists_alphabetically(ghost_print_final_list, input);
        let answer = vec![
            vec!["a".to_string(), "b".to_string()],
            vec!["c".to_string(), "d".to_string()],
        ];
        assert_eq!(output, answer);
    }

    #[test]
    fn test_print_final_list() {
        let input = vec![
            vec!["a".to_string(), "b".to_string()],
            vec!["c".to_string(), "d".to_string()],
        ];
        let answer = input.clone();
        let output = print_final_list(no_op, input);
        assert_eq!(output, answer);
    }
}
