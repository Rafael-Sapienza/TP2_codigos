use std::fs;
use std::fs::File;
use std::io::{BufReader};
use std::io::BufRead;
//use std::path::Path;
use std::collections::HashMap;


fn input_to_string(input_txt: String) -> String 
{
    match fs::read_to_string(&input_txt)
    {
        Ok(conteudo) => conteudo,
        Err(e) =>
        {
            panic!("Erro ao ler o arquivo '{}': {}", input_txt, e);
        },
    }
}

fn stop_words_file_to_vector(stop_words_txt: String) -> Vec<String>
{
    let mut stop_words_list:Vec<String> = Vec::new(); 
    let stop_words_file = File::open(&stop_words_txt);
    match stop_words_file
    {
        Ok(stop_words_file) => 
        {
            let reader = BufReader::new(stop_words_file);
            for linha in reader.lines()
            {
                match linha
                {
                    Ok(palavra) =>
                    {
                        let single_stop_word:String = palavra.trim().to_string();
                        stop_words_list.push(single_stop_word);
                    }
                    Err(e) => 
                    {
                        panic!("Erro ao ler o arquivo '{}': {}", stop_words_txt, e);
                    }
                }
            }
        },
        Err(e) =>
        {
            panic!("Erro ao ler o arquivo '{}': {}", stop_words_txt, e);
        },
    }
    stop_words_list
} 

fn input_string_to_vector(input_string: String) -> Vec<String>
{
    let mut all_words: Vec<String> = Vec::new();
    let mut  palavra: String = String::new();
    for caractere in input_string.to_lowercase().chars()
    {
        if caractere.is_alphanumeric()
        {
            palavra.push(caractere.clone());
        }
        else
        {
            if ! (palavra.is_empty()) 
            {
                all_words.push(palavra.clone());
                palavra.clear();
            }
        }
    }
    all_words
}

fn count_freq(all_words: Vec<String>, stop_words: Vec<String>) -> HashMap<String, u64>
{
    let mut dict_word_freq: HashMap<String, u64> = HashMap::new();
    for word in all_words
    {
        if ! stop_words.contains(&word)
        {
            dict_word_freq.entry(word.clone()).and_modify(|v| *v += 1).or_insert(1);
        }
    }
    dict_word_freq
}

fn sort_freq(dict_word_freq: HashMap<String,u64>) -> Vec<(String,u64)>
{
    // Ordena valores em ordem decrescente de frequencia
    let mut vector_word_freq: Vec<_> = dict_word_freq.into_iter().collect();
    vector_word_freq.sort_by(|a, b| (b.1).cmp(&a.1));
    vector_word_freq
}

fn print_most_frequent_words(number_of_words: u64, vector_word_freq: Vec<(String,u64)>)
{
    if number_of_words > vector_word_freq.len() as u64
    {
        println!("Número de palavras insuficiente!")
    }
    for i in 0..number_of_words
    {
        println! ("palavra: {}, frequência: {}", vector_word_freq[i as usize].0, vector_word_freq[i as usize].1);
    }
}

fn main()
{
    let input_string: String = input_to_string("../exemplo1/input.txt".to_string());
    let all_words:Vec<String> = input_string_to_vector(input_string);
    let stop_words_vector:Vec<String> = stop_words_file_to_vector("../exemplo1/stop_words.txt".to_string());
    let dict_word_freq:HashMap<String,u64> = count_freq(all_words, stop_words_vector);
    let vector_word_freq:Vec<(String,u64)> = sort_freq(dict_word_freq);
    print_most_frequent_words(10, vector_word_freq);
}
