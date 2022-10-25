use rand::seq::SliceRandom;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::BufRead;
mod errors;

struct MarkovChain {
    hash_map: HashMap<String, Vec<String>>,
    starting_ngrams: Vec<String>,
    n_before: usize,
    n_after: usize,
    min_len: usize,
    max_len: usize,
}

impl MarkovChain {
    fn new(n_b: usize, n_a: usize) -> Self {
        assert!(n_b >= 1 && n_a >= 1);
        MarkovChain {
            hash_map: HashMap::new(),
            starting_ngrams: Vec::new(),
            n_before: n_b,
            n_after: n_a,
            min_len: usize::MAX,
            max_len: usize::MIN,
        }
    }

    fn add(&mut self, n_gram_before: String, n_gram_after: String) {
        let n_grams_after = self.hash_map.entry(n_gram_before).or_insert(vec![]);
        n_grams_after.push(n_gram_after);
    }

    fn add_word(&mut self, word: &str) -> Result<(), errors::WordError> {
        let word_size = word.chars().count();
        if word_size < self.n_before + self.n_after {
            return Err(errors::WordError::TooSmall(
                format!(
                    "word of length '{}' is smaller than the minimum length of '{}'",
                    word_size,
                    self.n_before + self.n_after,
                )
                .to_owned(),
            ));
        }
        self.min_len = std::cmp::min(self.min_len, word_size);
        self.max_len = std::cmp::max(self.max_len, word_size);
        for i in 0..word_size - self.n_after - self.n_before + 1 {
            let before: String = word
                .chars()
                .enumerate()
                .filter(|(idx, _)| (i..i + self.n_before).contains(idx))
                .map(|(_, c)| c)
                .collect();

            if i == 0 {
                self.starting_ngrams.push(before.clone());
            }
            let after: String = word
                .chars()
                .enumerate()
                .filter(|(idx, _)| {
                    (i + self.n_before..i + self.n_before + self.n_after).contains(idx)
                })
                .map(|(_, c)| c)
                .collect();

            self.add(before, after);
        }

        Ok(())
    }

    fn generate_random_word(&self, len: usize) -> Result<String, errors::RequestedLengthError> {
        if self.starting_ngrams.is_empty() {
            panic!("No states added to the Markov Chain");
        }
        match len.cmp(&self.min_len) {
            Ordering::Less => {
                return Err(errors::RequestedLengthError::TooSmall(
                    format!(
                        "requested len '{}' is smaller than the minimum length of '{}'",
                        len, self.min_len,
                    )
                    .to_owned(),
                ))
            }
            _ => (),
        }

        match len.cmp(&self.max_len) {
            Ordering::Greater => {
                return Err(errors::RequestedLengthError::TooSmall(
                    format!(
                        "requested len '{}' is greater than the maximum length of '{}'",
                        len, self.max_len,
                    )
                    .to_owned(),
                ))
            }
            _ => (),
        }

        let mut word = self
            .starting_ngrams
            .choose(&mut rand::thread_rng())
            .unwrap()
            .clone();

        let mut word_size = self.n_before;
        let size = len / self.n_after;
        for _ in self.n_before..size + 1 {
            let n_gram_find: String = word
                .chars()
                .enumerate()
                .filter(|(idx, _)| (word_size - self.n_before..word_size).contains(idx))
                .map(|(_, c)| c)
                .collect();

            if !self.hash_map.contains_key(&n_gram_find) {
                break;
            }
            let random_choice = self.hash_map[&n_gram_find]
                .choose(&mut rand::thread_rng())
                .unwrap();
            word.push_str(&random_choice[..]);
            word_size += self.n_after;
        }
        Ok(word)
    }
}
fn main() {
    let file_name = "jp_cities.txt";

    let file = match fs::File::open(file_name) {
        Ok(file) => file,
        Err(error) => panic!("File could not be opened: {:?}", error),
    };
    let reader = io::BufReader::new(file);

    let mut chain = MarkovChain::new(2, 2);
    for line in reader.lines() {
        let name = match line {
            Ok(city) => city.to_lowercase(),
            Err(error) => panic!("Error: {:?}", error),
        };
        chain.add_word(&name);
    }
    for _ in 0..20 {
        let w = chain.generate_random_word(10);
        println!("{}", w.unwrap());
    }
}
