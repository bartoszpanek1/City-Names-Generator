use rand::seq::SliceRandom;
use std::cmp::Ordering;
use std::collections::HashMap;
use rand::Rng;

/// Struct used for city names generation
/// Should be constructed using CityGenerator::new(n_b, n_a) function
pub struct CityGenerator {
    hash_map: HashMap<String, Vec<String>>,
    starting_ngrams: Vec<String>,
    n_before: usize,
    n_after: usize,
    min_len: usize,
    max_len: usize,
}

impl CityGenerator {
    /// Constructs new City Generator
    ///
    /// n_b - beginning state (ngram) size
    /// n_a - other state (ngram) size
    ///
    /// # Examples
    /// Generate one letter basing on two previous letters
    /// n_b = 2, n_a = 1
    ///
    /// Generate two letters basing on two previous letters
    /// n_b = 2, n_a = 1
    pub fn new(n_b: usize, n_a: usize) -> Self {
        assert!(n_b >= 1 && n_a >= 1);
        CityGenerator {
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

    /// Add a new word to the generator
    /// Trains Markov Chain using provided word
    pub fn add_word(&mut self, word: &str) -> Result<(), crate::errors::WordError> {
        let word_size = word.chars().count();
        if word_size < self.n_before + self.n_after {
            return Err(crate::errors::WordError::TooSmall(
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

    /// Generates random word with size in range [self.min_len, self.max_len]
    /// Uses Markov Chain for generation
    pub fn generate_random_word(
        &self,
    ) -> Result<String, crate::errors::ChainError> {
        if self.starting_ngrams.is_empty() {
            return Err(crate::errors::ChainError::Empty("Markov has no states provided".to_owned()));
        }
        let len = rand::thread_rng().gen_range(self.get_min_len()..self.get_max_len()+1);

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

    pub fn get_min_len(&self) -> usize {
        self.min_len
    }

    pub fn get_max_len(&self) -> usize {
        self.max_len
    }

}
