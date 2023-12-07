use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};

fn split_into_words(content: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut split = content.split_whitespace();

    while let Some(word) = split.next() {
        let only_the_word = word
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect::<String>();
        if !only_the_word.is_empty() {
            words.push(only_the_word);
        }
    }

    words
}

fn main() -> io::Result<()> {
    let filename = r"C:\Users\Andreea\OneDrive\Desktop\RUST\lab9\p1\src\text.txt";

    let mut file_content = String::new();
    if let Ok(mut file) = File::open(filename) {
        file.read_to_string(&mut file_content)?;
    } else {
        println!("Error reading the file");
        return Ok(());
    }

    let mut word_count: HashMap<String, u32> = HashMap::new();
    for word in split_into_words(&file_content) {
        let counter = word_count.entry(word.to_lowercase()).or_insert(0);
        *counter += 1;
    }

    let mut sorted_word_count: Vec<_> = word_count.into_iter().collect();
    sorted_word_count.sort_by(|a, b| b.1.cmp(&a.1));

    for (word, count) in sorted_word_count {
        println!("{}: {}", word, count);
    }

    Ok(())
}
