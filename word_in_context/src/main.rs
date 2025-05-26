use std::fs;
use std::fs::File;
use std::io::{BufReader};
use std::io::BufRead;
//use std::path::Path;
use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp::Ordering;
use std::cmp::min;


fn stop_words_file_to_set(stop_words_txt: String) -> HashSet<String>
{
    let mut stop_words_set:HashSet <String> = HashSet::new(); 
    let stop_words_file = File::open(&stop_words_txt);
    match stop_words_file
    {
        Ok(stop_words_file) => 
        {
            let reader = BufReader::new(stop_words_file);
            for line in reader.lines()
            {
                match line
                {
                    Ok(word) =>
                    {
                        let single_stop_word:String = word.trim().to_string();
                        stop_words_set.insert(single_stop_word);
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
    stop_words_set
} 

fn input_file_to_string(input_txt: String) -> Vec<Vec<String>>
{
    let mut input_vector:Vec<Vec<String>> = Vec::new();
    let input_file = File::open(&input_txt);
    match input_file
    {
        Ok(input_file) =>
        {
            let reader = BufReader::new(input_file);
            for line in reader.lines()
            {
                match line
                {
                    Ok(line) =>
                    {
                        input_vector.push(Vec::new());
                        let mut word : String = String::new();
                        if let Some(current_line) = input_vector.last_mut()
                        {
                            for caractere in line.chars()
                            {
                                if caractere.is_alphanumeric()
                                {
                                    word.push(caractere.clone());
                                }
                                else if ! (word.is_empty())
                                {
                                    current_line.push(word.clone());
                                    word.clear();
                                }
                            }
                            if !word.is_empty()
                            {
                                current_line.push(word);
                            }
                        }
                    }
                    Err(e) => panic!("Erro ao ler linha"),
                }
            }
        }
        Err(e) => panic!("Erro ao ler o arquivo '{}' : {}", input_txt,e),
    }
    input_vector
}

/*
fn separate_stop_and_key_words(input_vector:Vec<Vec<String>>, stop_words_vector:Vec<String>) -> (Vec<Vec<String>>,Vec<Vec<String>>)
{
    let mut stop_words_occurrences : Vec<Vec<String>> = vec![Vec::new();input_vector.len()];
    let mut key_words_occurrences :  Vec<Vec<String>> = vec![Vec::new();input_vector.len()];
    for i in 0..input_vector.len()
    {
        for string in &input_vector[i]
        {
            if stop_words_vector.contains(&string.to_lowercase())
            {
                stop_words_occurrences[i].push(string.clone());
            }
            else
            {
                key_words_occurrences[i].push(string.clone())
            }
        }
    }
    (stop_words_occurrences,key_words_occurrences)
}
*/


fn find_key_words_indexes(input_vector : &Vec<Vec<String>>, stop_words_set : &HashSet<String>) -> Vec<Vec<u64>>
{
    let mut key_words_occurrences :  Vec<Vec<u64>> = vec![Vec::new();input_vector.len()];
    for (i,line) in input_vector.iter().enumerate()
    {
        for (j,string) in line.iter().enumerate()
        {
            let string_aux = string.to_lowercase();
            if !stop_words_set.contains(&string_aux)
            {
                key_words_occurrences[i].push(j as u64)
            }
        }
    }
    key_words_occurrences
} 

fn generate_circularly_shifted_lists(input_vector:Vec<Vec<String>>,key_words_occurrences:Vec<Vec<u64>>) -> Vec<Vec<String>>
{
    let mut circularly_shifted_lists : Vec<Vec<String>> = Vec::new();
    for (i,line) in input_vector.iter().enumerate()
    {
        for j in  &key_words_occurrences[i]
        {
            let j = *j as usize;
            {
                let before = &line[..j];
                let after = &line[j..];
                let circular_shifted_list = 
                after
                .iter()
                .chain(before.iter())
                .cloned()
                .collect();
                circularly_shifted_lists.push(circular_shifted_list);
            }
        }
    }
    circularly_shifted_lists
}

fn compare_alphabetically(a:&Vec<String>,b:&Vec<String>) -> Ordering
{
    if a.is_empty() || a[0].is_empty()
    {
        return Ordering::Less;
    }
    if b.is_empty() || b[0].is_empty()
    {
        return Ordering::Greater;
    }
    for i in 0..min(a.len(),b.len())
    {
        let first_char = a[i]
                        .chars()
                        .next()
                        .unwrap()
                        .to_lowercase()
                        .next()
                        .unwrap();
        let second_char = b[i]
                        .chars()
                        .next()
                        .unwrap()
                        .to_lowercase()
                        .next()
                        .unwrap();
        let order:Ordering = first_char.cmp(&second_char);
        if order != Ordering::Equal
        {
            return order;
        }
    }
    //Desempata pelo comprimento de vetores (numero de palavras no titulo)
    a.len().cmp(&b.len())
}


fn sort_circularly_shifted_lists_alphabetically(mut circularly_shifted_lists:Vec<Vec<String>>) -> Vec<Vec<String>>
{
    circularly_shifted_lists.sort_by(|a,b| compare_alphabetically(a,b));
    circularly_shifted_lists
}

fn main()
{
    let input_vector: Vec<Vec<String>> = input_file_to_string("../input_exemplo_3/input.txt".to_string());
    let stop_words_set:HashSet<String> = stop_words_file_to_set("../input_exemplo_3/stop_words.txt".to_string());
    let key_words_occurrences:Vec<Vec<u64>> = find_key_words_indexes(&input_vector,&stop_words_set);
    //println!("{:?}",key_words_occurrences);
    let circularly_shifted_lists:Vec<Vec<String>> = generate_circularly_shifted_lists(input_vector,key_words_occurrences);
    println!("circularly_shifted_lists:");
    println!("{:?}",circularly_shifted_lists);
    let sorted_final_list : Vec<Vec<String>> = sort_circularly_shifted_lists_alphabetically(circularly_shifted_lists);
    println!("sorted final list:");
    println!("{:?}",sorted_final_list);
}