mod word;
pub mod anagram;

use std::fs::File;
use std::str::FromStr;
use std::io::{prelude::*, BufReader};

pub use crate::word::*;

pub struct Verve {
    pub word_data: Vec<WordDatum>
}

impl Verve {
    pub fn new() -> Self {
        let dict_str = include_str!("res/enable1.txt");
        let mut word_data = Vec::new();
        for (i, line) in dict_str.lines().enumerate() {
            let word = Word::from_str(
                &line.to_lowercase()
            ).expect("word parsing failed");
            word_data.push(WordDatum { word, id : i });
        }
        return Verve { word_data };
    }
    pub fn new_from(dictpath : &str) -> Self {
        let file = File::open(dictpath)
            .expect("failed to read file");
        let reader = BufReader::new(file);

        let mut word_data = Vec::new();
        for (i, line) in reader.lines().enumerate() {
            let word = Word::from_str(
                &line.unwrap().to_lowercase()
            ).expect("word parsing failed");
            word_data.push(WordDatum { word, id : i });
        }
        return Verve { word_data };
    }
    pub fn word(&self, id : usize) -> &Word {
        let word_datum = &self.word_data[id];
        assert_eq!(word_datum.id, id);
        return &word_datum.word;
    }
}
