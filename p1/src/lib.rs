use std::collections::HashSet;

/// Tests if a pair of numbers in a slice sum to a value.
///
/// Returns true if a pair sum to k.
///
/// # Examples
///
/// ```
/// use p1::has_pair_with_sum_nested_loops;
///
/// assert!(has_pair_with_sum_nested_loops(&[10, 15, 3, 7], 17));
/// ```
pub fn has_pair_with_sum_nested_loops(numbers: &[u32], k: u32) -> bool {
    for i in 0..numbers.len() {
        for j in (i + 1)..numbers.len() {
            if numbers[i] + numbers[j] == k {
                return true;
            }
        }
    }
    false
}

/// Tests if a pair of numbers in a slice sum to a value.
///
/// Returns true if a pair sum to k.
///
/// # Examples
///
/// ```
/// use p1::has_pair_with_sum_set;
///
/// assert!(has_pair_with_sum_set(&[10, 15, 3, 7], 17));
/// ```
pub fn has_pair_with_sum_set(numbers: &[u32], k: u32) -> bool {
    let mut set = HashSet::new();
    for n in numbers {
        if *n > k {
            continue;
        }
        if set.contains(&(k - n)) {
            return true;
        }
        set.insert(n);
    }
    false
}

mod test {
    #[test]
    fn test_nested_loops() {
        use has_pair_with_sum_nested_loops;
        let numbers: [u32; 4] = [10, 15, 3, 7];
        assert!(has_pair_with_sum_nested_loops(&numbers, 18));
        assert!(has_pair_with_sum_nested_loops(&numbers, 18));
        assert!(has_pair_with_sum_nested_loops(&numbers, 25));
        assert!(!has_pair_with_sum_nested_loops(&numbers, 3));
    }

    #[test]
    fn test_set() {
        use has_pair_with_sum_set;
        let numbers: [u32; 4] = [10, 15, 3, 7];
        assert!(has_pair_with_sum_set(&numbers, 18));
        assert!(has_pair_with_sum_set(&numbers, 18));
        assert!(has_pair_with_sum_set(&numbers, 25));
        assert!(!has_pair_with_sum_set(&numbers, 3));
    }

}
