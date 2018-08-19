/* 
Good morning! Here's your coding interview problem for today.
This problem was asked by Twitter.

Implement an autocomplete system. That is, given a query string s and a set of
all possible query strings, return all strings in the set that have s as a
prefix.

For example, given the query string de and the set of strings [dog, deer, deal],
return [deer, deal].

Hint: Try preprocessing the dictionary into a more efficient data structure to
speed up queries.
*/
use std::collections::{HashMap, HashSet};

fn strings_with_prefix<'a>(prefix: &str, strings: &'a [&str]) -> HashSet<&'a str> {
    strings
        .to_vec()
        .into_iter()
        .filter(|string| string.starts_with(prefix))
        .collect()
}

fn strings_with_prefix_using_trie(prefix: &str, strings: &[&str]) -> HashSet<String> {
    let mut set = TrieSet::new();
    for word in strings {
        set.insert(&word);
    }
    set.words_starting_with(prefix)
}

#[derive(Debug)]
struct Node {
    terminates: bool,
    chars: HashMap<char, Node>,
}

#[derive(Debug)]
pub struct TrieSet {
    root: Node,
}

impl TrieSet {
    fn new() -> Self {
        TrieSet {
            root: Node {
                terminates: false,
                chars: HashMap::new(),
            },
        }
    }

    fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        for c in word.chars() {
            node = moving(node).chars.entry(c).or_insert(Node {
                terminates: false,
                chars: HashMap::new(),
            });
        }
        node.terminates = true;
    }

    fn contains(&self, word: &str) -> bool {
        let mut node = &self.root;
        for c in word.chars() {
            match node.chars.get(&c) {
                None => return false,
                Some(next) => node = next,
            }
        }
        node.terminates
    }

    pub fn words_starting_with(&self, prefix: &str) -> HashSet<String> {
        let mut node = &self.root;
        for c in prefix.chars() {
            match node.chars.get(&c) {
                None => return HashSet::new(),
                Some(next) => node = next,
            }
        }
        let mut words = HashSet::new();
        self.collect_words(&node, prefix, &mut words);
        words
    }

    fn collect_words(&self, node: &Node, prefix: &str, words: &mut HashSet<String>) {
        if node.terminates {
            words.insert(String::from(prefix));
        }
        for (c, child_node) in &node.chars {
            // Traverse each branch now
            let mut prefix = String::from(prefix);
            prefix.push(*c);
            self.collect_words(&child_node, &prefix, words);
        }
    }
}

fn moving<T>(t: T) -> T {
    t
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strings_with_prefix() {
        let mut matches = HashSet::new();
        matches.insert("deer");
        matches.insert("deal");
        assert_eq!(strings_with_prefix("de", &["dog", "deer", "deal"]), matches);
    }

    #[test]
    fn test_string_with_prefix_using_trie() {
        let mut matches = HashSet::new();
        matches.insert("deer".to_string());
        matches.insert("deal".to_string());
        assert_eq!(
            strings_with_prefix_using_trie("de", &["dog", "deer", "deal"]),
            matches
        );
    }
}
