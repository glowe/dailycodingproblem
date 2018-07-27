/// Computes a Vec of products.
///
/// Each element of the Vec is the product of all the elements in the slice
/// except for the element at the corresponding position.
///
/// Examples:
/// ```
/// use p2::products_with_division;
/// assert_eq!(products_with_division(&[1, 2, 3, 4, 5]), [120, 60, 40, 30, 24]);
/// ```
pub fn products_with_division(numbers: &[u32]) -> Vec<u32> {
    let product_of_all: u32 = numbers.iter().product();
    numbers.iter().map(|n| product_of_all / n).collect()
}

/// Computes a Vec of products.
///
/// Each element of the Vec is the product of all the elements in the slice
/// except for the element at the corresponding position.
///
/// Examples:
/// ```
/// use p2::products_without_division;
/// assert_eq!(products_without_division(&[1, 2, 3, 4, 5]), [120, 60, 40, 30, 24]);
/// ```
pub fn products_without_division(numbers: &[u32]) -> Vec<u32> {
    (0..numbers.len())
        .map(|i| {
            numbers
                .iter()
                .take(i)
                .chain(numbers.iter().skip(i + 1))
                .product()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_with_division() {
        use products_with_division;
        assert_eq!(
            products_with_division(&[1, 2, 3, 4, 5]),
            [120, 60, 40, 30, 24]
        );
        assert_eq!(products_with_division(&[3, 2, 1]), [2, 3, 6]);
    }

    #[test]
    fn test_without_division() {
        use products_without_division;
        assert_eq!(
            products_without_division(&[1, 2, 3, 4, 5]),
            [120, 60, 40, 30, 24]
        );
        assert_eq!(products_without_division(&[3, 2, 1]), [2, 3, 6]);
    }
}
