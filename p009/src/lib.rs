use std::cmp;

fn largest_sum(numbers: &[i64]) -> i64 {
    let mut including_previous = 0;
    let mut excluding_previous = 0;
    for n in numbers {
        let next_excluding_previous = cmp::max(excluding_previous, including_previous);
        including_previous = excluding_previous + n;
        excluding_previous = next_excluding_previous;
    }
    cmp::max(including_previous, excluding_previous)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_example() {
        let numbers = [2, 4, 6, 2, 5];
        assert_eq!(largest_sum(&numbers), 13);
    }

    #[test]
    fn test_second_example() {
        let numbers = [5, 1, 1, 5];
        assert_eq!(largest_sum(&numbers), 10);
    }

    #[test]
    fn test_more_complicated() {
        let numbers = [1, 5, 1, 1, 6, 5, 3];
        assert_eq!(largest_sum(&numbers), 14);
    }

    #[test]
    fn test_example_with_negatives() {
        let numbers = [10, -10, -20, -30, -5, 9];
        assert_eq!(largest_sum(&numbers), 19);
    }
}
