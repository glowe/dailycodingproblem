/* 
 This problem was asked by Amazon.
 There exists a staircase with N steps, and you can climb up either 1 or 2
 steps at a time. Given N, write a function that returns the number of
 unique ways you can climb the staircase. The order of the steps matters. 

 For example, if N is 4, then there are 5 unique ways: 
 
 * 1, 1, 1, 1 
 * 2, 1, 1 
 * 1, 2, 1 
 * 1, 1, 2 
 * 2, 2 
 
 What if, instead of being able to climb 1 or 2 steps at a time, you could 
 climb any number from a set of positive integers X? For example, if 
 X = {1, 3, 5}, you could climb 1, 3, or 5 steps at a time. 
*/
use std::collections::HashSet;

fn permutations_lte_sum(choices: &[u8], sum: u8, m: u8) -> Vec<Vec<u8>> {
    if m == 0 {
        let mut vec = Vec::new();
        vec.push(Vec::new());
        return vec;
    }
    let mut perms = Vec::new();
    for perm in permutations_lte_sum(choices, sum, m - 1) {
        for choice in choices {
            let mut perm = perm.clone();
            let perm_total: u8 = perm.iter().sum();
            if perm_total + choice <= sum {
                perm.insert(0, *choice);
            }
            perms.push(perm);
        }
    }
    perms
}

fn staircase_permutations(choices: &[u8], sum: u8) -> usize {
    let mut set = HashSet::new();
    for perm in permutations_lte_sum(choices, sum, sum) {
        let perm_total: u8 = perm.iter().sum();
        if perm_total == sum {
            set.insert(perm);
        }
    }
    set.len()
}

fn staircase_exponential(choices: &[u8], n: u8) -> u8 {
    if n == 0 {
        1
    } else if choices.contains(&n) {
        1 + choices
            .iter()
            .filter(|&&c| c < n)
            .map(|&c| staircase_exponential(choices, n - c))
            .sum::<u8>()
    } else {
        choices
            .iter()
            .filter(|&&c| c < n)
            .map(|&c| staircase_exponential(choices, n - c))
            .sum::<u8>()
    }
}

fn staircase_linear_x(choices: &[u8], n: u8) -> u8 {
    let n: usize = n as usize;
    let mut cache = vec![0; n + 1];
    cache[0] = 1;
    for i in 0..=n {
        cache[i] += choices
            .iter()
            .filter(|&&c| i as i32 - c as i32 > 0)
            .map(|&c| cache[i - c as usize])
            .sum::<u8>();
        if choices.contains(&(i as u8)) {
            cache[i] += 1;
        }
    }
    *cache.last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_staircase_permutations() {
        assert_eq!(staircase_permutations(&[1, 2], 4), 5);
    }

    #[test]
    fn test_staircase_exponential() {
        assert_eq!(staircase_exponential(&[1, 2], 4), 5);
    }

    #[test]
    fn test_staircase_linear_x() {
        assert_eq!(staircase_linear_x(&[1, 2], 4), 5);
    }
}
