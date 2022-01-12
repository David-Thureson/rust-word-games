use crate::*;
use rand::thread_rng;
use rand::seq::SliceRandom;

pub struct Puzzle {
    phrases: Vec<Phrase>,
    blank_count: usize,
}

pub struct Phrase {
    phrase: String,
    jumble: String,
    missing_chars: Vec<char>,
    char_indexes: Vec<usize>,
}

impl Puzzle {
    pub fn new(phrases: &[&str], blank_count: usize) -> Self {
        let mut phrases = phrases.iter().map(|phrase| Phrase::new(phrase)).collect::<Vec<_>>();
        phrases.shuffle(&mut thread_rng());
        Self {
            phrases,
            blank_count,
        }
    }

    pub fn build(&mut self) {
        for phrase in self.phrases.iter_mut() {
            phrase.build(self.blank_count);
        }
    }

    pub fn try_jumbles(phrases: &[&str], blank_count: usize, try_count_max: usize) {
        for _ in 0..try_count_max {
            let mut puzzle = Puzzle::new(phrases, blank_count);
            puzzle.build();
            puzzle.print(true);
        }
    }

    pub fn print(&self, show_phrase: bool) {
        println!("\n=============================================================================\n");
        for phrase in self.phrases.iter() {
            if show_phrase {
                println!("{}", phrase.phrase);
            }
            let missing_chars = phrase.missing_chars.iter().join(" ");
            println!("{}", missing_chars);
            println!("{}\n", phrase.jumble);
        }
        println!("\n-----------------------------------------------------------------------------\n");
    }
}

impl Phrase {
    pub fn new(phrase: &str) -> Self {
        let mut char_indexes = vec![];
        for (i, c) in phrase.chars().enumerate() {
            if c.is_alphabetic() {
                char_indexes.push(i);
            }
        }
        Self {
            phrase: phrase.to_string(),
            jumble: "".to_string(),
            missing_chars: vec![],
            char_indexes,
        }
    }

    fn build(&mut self, blank_count: usize) {
        let try_count_for_error = 100;
        let mut try_count = 0;
        let blank_count = blank_count.min(self.phrase.len() - 1);
        loop {
            self.char_indexes.shuffle(&mut thread_rng());
            self.jumble = "".to_string();
            self.missing_chars.clear();
            for (i, c) in self.phrase.chars().enumerate() {
                if self.char_indexes[..blank_count].contains(&i) {
                    self.jumble.push('_');
                    self.missing_chars.push(c);
                } else {
                    self.jumble.push(c);
                }
            }
            let mut test_missing_chars = self.missing_chars.clone();
            test_missing_chars.sort();
            test_missing_chars.dedup();
            if test_missing_chars.len() >= 2 {
                self.missing_chars.shuffle(&mut thread_rng());
                return;
            }
            try_count += 1;
            if try_count >= try_count_for_error {
                panic!();
            }
       }
    }
}

pub fn main() {
    let phrases = word_list::WORDS_1;
    let blank_count = 2;
    let try_count_max = 5;
    Puzzle::try_jumbles(&phrases, blank_count, try_count_max);
}