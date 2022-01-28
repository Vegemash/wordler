use std::fs;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::str::Chars;

fn read_lines<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = fs::File::open(&filename).unwrap();
    let reader = BufReader::new(&file);
    reader.lines().collect::<Result<_, _>>().unwrap()
}

fn chars_filtered_exclusive(words: Vec<String>, chars: Chars) -> Vec<String> {
    return words
        .clone()
        .iter()
        .filter(|&x| chars.clone().filter(|&c| x.contains(c)).count() == 0)
        .map(String::to_string)
        .collect::<Vec<String>>();
}

fn chars_filtered_inclusive(words: Vec<String>, chars: Chars) -> Vec<String> {
    return words
        .clone()
        .iter()
        .filter(|&x| chars.clone().filter(|&c| x.contains(c)).count() == chars.clone().count())
        .map(String::to_string)
        .collect::<Vec<String>>();
}

fn n_char_filtered_in(words: Vec<String>, ch: char, n: usize) -> Vec<String> {
    return words
        .clone()
        .iter()
        .filter(|&x| x.as_bytes()[n] as char == ch)
        .map(String::to_string)
        .collect::<Vec<String>>();
}

fn n_char_filtered_ex(words: Vec<String>, ch: char, n: usize) -> Vec<String> {
    return words
        .clone()
        .iter()
        .filter(|&x| x.as_bytes()[n] as char != ch)
        .map(String::to_string)
        .collect::<Vec<String>>();
}

fn main() {
    let mut words = read_lines("/usr/share/dict/words")
        .iter()
        .filter(|&x| x.chars().count() == 5)
        .map(|x| x.to_string().to_lowercase())
        .collect::<Vec<String>>();
    words.dedup();
    words = chars_filtered_exclusive(words, "-'.".chars());
    words = chars_filtered_exclusive(words, "sarecalpi".chars());
    words = chars_filtered_inclusive(words, "ot".chars());

    words = n_char_filtered_ex(words, 't', 1);

    words = n_char_filtered_ex(words, 'o', 0);
    words = n_char_filtered_ex(words, 't', 2);

    words = n_char_filtered_in(words, 'o', 1);
    words = n_char_filtered_in(words, 'n', 3);
    words = n_char_filtered_in(words, 't', 4);

    println!("{:?}, {}", words, words.iter().count());
    // TODO: Implement console input
    // TODO: Reduce clones
    // TODO: Turn functions into trait functions
    // TODO: Implement all() and any() methods
}
