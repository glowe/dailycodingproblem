/*
Good morning! Here's your coding interview problem for today. 
 
This problem was asked by Amazon. 
 
Given an integer k and a string s, find the length of the longest substring that 
contains at most k distinct characters. 
 
For example, given s = "abcba" and k = 2, the longest substring with k distinct 
characters is "bcb".
*/
use std::collections::HashSet;

fn longest_substring<'a>(k: u8, s: &'a str) -> &'a str {
    let mut longest = &s[0..0];
    for i in 0..s.len() {
        let mut chars = HashSet::new();
        for j in i..s.len() {
            chars.insert(s.chars().nth(j));
            if chars.len() > k as usize {
                break;
            }
            let substring = &s[i..j + 1];
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
    fn test_longest_substring() {
        assert_eq!(longest_substring(2, "abcba"), "bcb");
        assert_eq!(longest_substring(2, "abdcc"), "dcc");
        assert_eq!(longest_substring(2, "aacba"), "aac");
        assert_eq!(longest_substring(2, "aaaba"), "aaaba");
        assert_eq!(longest_substring(2, "abcde"), "ab");
    }
}
