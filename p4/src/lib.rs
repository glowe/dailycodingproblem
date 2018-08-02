fn find_smallest_missing_positive_int_quadratic(numbers: &[i32]) -> Option<u32> {
    for i in 1..numbers.len() + 1 {
        let mut found = false;
        for n in numbers {
            if *n == i as i32 {
                found = true;
                break;
            }
        }
        if !found {
            return Some(i as u32);
        }
    }
    None
}

fn find_smallest_missing_positive_int_nlogn(numbers: &mut [i32]) -> Option<u32> {
    numbers.sort();
    let mut smallest = 1;
    for n in numbers.into_iter() {
        if *n < smallest {
            continue;
        } else if *n == smallest {
            smallest = *n + 1;
        } else if *n > smallest {
            return Some(smallest as u32);
        }
    }
    if smallest == (numbers.len() + 1) as i32 {
        None
    } else {
        Some(smallest as u32)
    }
}

fn find_smallest_missing_positive_int_linear(numbers: &mut [i32]) -> Option<u32> {
    let max = numbers.len();

    // Move all of the non-positives out of consideration
    for n in numbers.iter_mut() {
        if *n <= 0 {
            *n = (max as i32) + 1;
        }
    }

    for i in 0..numbers.len() {
        let n = numbers[i].abs() as usize;
        assert!(n > 0);
        if n <= max && numbers[n - 1] > 0 {
            numbers[n - 1] *= -1;
        }
    }

    for i in 0..numbers.len() {
        if numbers[i] > 0 {
            return Some((i + 1) as u32);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_quadratic() {
        assert_eq!(
            find_smallest_missing_positive_int_quadratic(&[3, 4, -1, 1]),
            Some(2)
        );
        assert_eq!(
            find_smallest_missing_positive_int_quadratic(&[1, 2, 0]),
            Some(3)
        );
        assert_eq!(
            find_smallest_missing_positive_int_quadratic(&mut [2, 3, 7, 6, 8, -1, -10, 15]),
            Some(1)
        );
        assert_eq!(
            find_smallest_missing_positive_int_quadratic(&mut [2, 3, -7, 6, 8, 1, -10, 15]),
            Some(4)
        );
        assert_eq!(
            find_smallest_missing_positive_int_quadratic(&mut [1, 1, 0, -1, -2]),
            Some(2)
        );
    }

    #[test]
    fn test_nlogn() {
        assert_eq!(
            find_smallest_missing_positive_int_nlogn(&mut [3, 4, -1, 1]),
            Some(2)
        );
        assert_eq!(
            find_smallest_missing_positive_int_nlogn(&mut [1, 2, 0]),
            Some(3)
        );
        assert_eq!(
            find_smallest_missing_positive_int_nlogn(&mut [2, 3, 7, 6, 8, -1, -10, 15]),
            Some(1)
        );
        assert_eq!(
            find_smallest_missing_positive_int_nlogn(&mut [2, 3, -7, 6, 8, 1, -10, 15]),
            Some(4)
        );
        assert_eq!(
            find_smallest_missing_positive_int_nlogn(&mut [1, 1, 0, -1, -2]),
            Some(2)
        );
    }

    #[test]
    fn test_linear() {
        assert_eq!(
            find_smallest_missing_positive_int_linear(&mut [3, 4, -1, 1]),
            Some(2)
        );
        assert_eq!(
            find_smallest_missing_positive_int_linear(&mut [1, 2, 0]),
            Some(3)
        );
        assert_eq!(
            find_smallest_missing_positive_int_linear(&mut [2, 3, 7, 6, 8, -1, -10, 15]),
            Some(1)
        );
        assert_eq!(
            find_smallest_missing_positive_int_linear(&mut [2, 3, -7, 6, 8, 1, -10, 15]),
            Some(4)
        );
        assert_eq!(
            find_smallest_missing_positive_int_linear(&mut [1, 1, 0, -1, -2]),
            Some(2)
        );
    }

}
