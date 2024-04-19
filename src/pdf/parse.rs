use regex::Regex;
use std::collections::HashMap;


// Returns an unsorted HashMap containing the word (string) and number of instances (usize).
#[deprecated = "The function better_parse_for_words has a better word detection algorithim. It is recommended to use that function instead of this one."]
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

// Uses regex and splits by whitespace to find words.
// Returns an unsorted HashMap containing the word (string) and number of isntances (usize)
pub fn regex_parse_for_words(data: &str) -> HashMap<String, usize> {
    let fixed_data = data.to_ascii_lowercase();
    let re = Regex::new(r"[^\w-]+").unwrap(); 
    let cleaned_data = re.replace_all(&fixed_data, " "); 

    let mut word_counts: HashMap<String, usize> = HashMap::new();

    let words: Vec<&str> = cleaned_data.split_whitespace().collect();

    for word in words {
        *word_counts.entry(word.to_string()).or_insert(0) += 1;
    }
    word_counts
}

// Returns a sorted vector from most instances to least instances for the words.
#[deprecated = "The function sort_by_instances_hash maintains HashMap type. Unless you need a Vec type, it is recommended to not use this function."]
pub fn sort_by_instances(data: HashMap<String, usize>) -> Vec<(String, usize)> {
    let mut sorted_entries: Vec<(String, usize)> = data.into_iter().collect();
    sorted_entries.sort_by(|a, b| b.1.cmp(&a.1));
    sorted_entries
}

pub fn sort_by_instances_hash(data: HashMap<String, usize>) -> HashMap<String, usize> {
    let mut hash: HashMap<String, usize> = HashMap::new();
    let mut sorted_entries: Vec<(String, usize)> = data.into_iter().collect();
    sorted_entries.sort_by(|a, b| b.1.cmp(&a.1));
    for (name, instances) in sorted_entries {
        &hash.entry(name).or_insert(instances);
    }
    hash

}
