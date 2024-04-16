use std::collections::{HashMap};
use regex::Regex;

// Returns an unsorted HashMap containing the word (string) and number of instances (usize).
pub fn parse_for_words(data: &String) -> HashMap<String, usize> {
  // make the string all lowercase for better word detection
  let fixed_data = data.to_ascii_lowercase();

  let mut word_counts: HashMap<String, usize> = HashMap::new();

  let words: Vec<&str> = fixed_data.split_whitespace().collect();

  for word in words {
    *word_counts.entry(word.to_string()).or_insert(0) += 1;
  }

  word_counts
}

// TODO: fix regex
pub fn better_parse_for_words(data: &str) -> HashMap<String, usize> {
  let fixed_data = data.to_ascii_lowercase();
  let re = Regex::new(r"[^\w-]+").unwrap(); // Matches any character that is not a word character or hyphen
  let cleaned_data = re.replace_all(&fixed_data, " "); // Replace punctuation with whitespace
  let mut word_counts: HashMap<String, usize> = HashMap::new();
  let words: Vec<&str> = cleaned_data.split_whitespace().collect();
  for word in words {
      *word_counts.entry(word.to_string()).or_insert(0) += 1;
  }
  word_counts
}

pub fn sort_by_instances(data: HashMap<String, usize>) -> Vec<(String, usize)> {
  let mut sorted_entries: Vec<(String, usize)> = data.into_iter().collect();
  sorted_entries.sort_by(|a, b| b.1.cmp(&a.1));
  sorted_entries
}