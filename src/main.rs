extern crate clap;
extern crate rand;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use rand::Rng;

use clap::{Arg, App};

fn main() {
    // get the name of the file that contains the word list
    // we'll use to generate passwords
    let filename = get_filename();

    // read the word file and read it into a list
    let word_vec = read_word_list(filename);

    // get the commandline arguments
    let args = App::new("Passwords")
                .version("1.0")
                .author("Gareth Fleming")
                .about("Generate passwords using dictionry words")
                .arg(Arg::with_name("num_words")
                    .short("n")
                    .long("num_words")
                    .value_name("num_words")
                    .help("The number of words in the password")
                    .takes_value(true))
                .arg(Arg::with_name("separator")
                    .short("s")
                    .long("separator")
                    .value_name("separator")
                    .help("Separator used between words in the password")
                    .takes_value(true))
                .get_matches();
    let num_words = args.value_of("num_words").unwrap_or("4")
        .parse()
        .expect("The number of words should be a numeric value.");
    let separator = args.value_of("separator").unwrap_or(",");

    let password = generate_password(word_vec, num_words, separator);
    println!("{}", password)
}

fn get_filename() -> String {
    match env::var("PASSWORDS_DICT_FILE") {
        Ok(file) => return file.to_string(),
        Err(_) => return "/usr/share/dict/words".to_string(),
    }
}

fn read_word_list<P>(filename: P) -> Vec<String> 
where
    P: AsRef<Path>
{
    let words_file = File::open(filename).expect("Cannot open word file");
    let buf = BufReader::new(words_file);
    return buf.lines()
              .map(|line| line.expect("Can't read line"))
              .collect();
}

fn generate_password(words: Vec<String>, num_words: u32, separator: &str) -> String {
    let mut password_vec = Vec::new();
    for _ in 0..num_words {
        let idx = rand::thread_rng().gen_range(0, words.len());
        password_vec.push(words[idx].clone());
    }
    return password_vec.join(separator);
}
