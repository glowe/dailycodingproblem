/*
Good morning! Here's your coding interview problem for today. 
 
This problem was asked by Amazon. 
 
Given an integer k and a string s, find the length of the longest substring that 
contains at most k distinct characters. 
 
For example, given s = "abcba" and k = 2, the longest substring with k distinct 
characters is "bcb".
*/
use std::collections::HashSet;

fn longest_substring_linear(s: &str, k: usize) -> &str {
    fn distance(slice: (usize, usize)) -> usize {
        slice.1 - slice.0
    }
    let mut chars = HashSet::new();
    let mut current = (0, 0);
    let mut best = (0, 0);
    for (i, ch) in s.chars().enumerate() {
        chars.insert(ch);
        if chars.len() > k {
            chars.remove(&s.chars().nth(current.0).unwrap());
            current.0 += 1;
        }
        current.1 = i + 1;
        if distance(current) > distance(best) {
            best = current;
        }
    }
    &s[best.0..best.1]
}

fn longest_substring_quadratic(s: &str, k: usize) -> &str {
    let mut longest = &s[0..0];
    for begin in 0..s.len() {
        let mut chars = HashSet::new();
        for end in begin..s.len() {
            chars.insert(s.chars().nth(end));
            if chars.len() > k {
                break;
            }
            let substring = &s[begin..end + 1];
            if substring.len() > longest.len() {
                longest = substring;
            }
        }
    }
    longest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_substring_quadratic() {
        assert_eq!(longest_substring_quadratic("abcba", 2), "bcb");
        assert_eq!(longest_substring_quadratic("abdcc", 2), "dcc");
        assert_eq!(longest_substring_quadratic("aacba", 2), "aac");
        assert_eq!(longest_substring_quadratic("aaaba", 2), "aaaba");
        assert_eq!(longest_substring_quadratic("abcde", 2), "ab");
    }

    #[test]
    fn test_longest_substring_linear() {
        assert_eq!(longest_substring_linear("abcba", 2), "bcb");
        assert_eq!(longest_substring_linear("abdcc", 2), "dcc");
        assert_eq!(longest_substring_linear("aacba", 2), "aac");
        assert_eq!(longest_substring_linear("aaaba", 2), "aaaba");
        assert_eq!(longest_substring_linear("abcde", 2), "ab");
    }

}
