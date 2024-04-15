use std::collections::{HashMap, BTreeMap};

pub fn parse_for_words(data: &String) -> HashMap<String, usize> {
  // make the string all lowercase for better word detection
  let mut word_counts: HashMap<String, usize> = HashMap::new();

  let words: Vec<&str> = data.split_whitespace().collect();

  for word in words {
    *word_counts.entry(word.to_string()).or_insert(0) += 1;
  }

  word_counts
}

pub fn sort_by_instances(data: HashMap<String, usize>) -> BTreeMap<usize, String> {
  let mut sorted_btree: BTreeMap<usize, String> = BTreeMap::new();

  for (word, count) in data {
      sorted_btree.insert(count, word);
  }

  sorted_btree
}