/*
Good morning! Here's your coding interview problem for today.

This problem was asked by Google.

Given an array of integers and a number k, where 1 <= k <= length of the array,
compute the maximum values of each subarray of length k.

For example, given array = [10, 5, 2, 7, 8, 7] and k = 3, we should get: [10, 7, 8, 8], since:

    10 = max(10, 5, 2)
    7 = max(5, 2, 7)
    8 = max(2, 7, 8)
    8 = max(7, 8, 7)

Do this in O(n) time and O(k) space. You can modify the input array in-place and you do not 
need to store the results. You can simply print them out as you compute them.
*/
use std::collections::VecDeque;

fn max_val_nk(arr: &[u64], k: usize) -> Vec<u64> {
    arr.windows(k).map(|s| *s.iter().max().unwrap()).collect()
}

fn max_val_linear(arr: &[u64], k: usize) -> Vec<u64> {
    let mut maxes = Vec::new();
    let mut q = VecDeque::with_capacity(k);
    for i in 0..k {
        // Remove elements from the back that are smaller
        while !q.is_empty() && arr[i] >= arr[*q.back().unwrap()] {
            q.pop_back();
        }
        q.push_back(i);
    }
    for i in k..arr.len() {
        maxes.push(arr[*q.front().unwrap()]);
        // Remove elements outside of window
        while !q.is_empty() && i - q.front().unwrap() >= k {
            q.pop_front();
        }
        // Remove elements from the back that are smaller
        while !q.is_empty() && arr[i] >= arr[*q.back().unwrap()] {
            q.pop_back();
        }
        q.push_back(i);
    }
    maxes.push(arr[*q.front().unwrap()]);
    maxes
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_val_nk() {
        assert_eq!(max_val_nk(&[10, 5, 2, 7, 8, 6], 3), vec![10, 7, 8, 8]);
    }

    #[test]
    fn test_max_val_linear() {
        assert_eq!(max_val_linear(&[10, 5, 2, 7, 8, 6], 3), vec![10, 7, 8, 8]);
    }
}
