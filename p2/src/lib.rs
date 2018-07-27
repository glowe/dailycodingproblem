/// Computes a Vec of products.
///
/// Each element of the Vec is the product of all the elements in the slice
/// except for the element at the corresponding position.
///
/// Examples:
/// ```
/// use p2::products;
/// assert_eq!(products(&[1, 2, 3, 4, 5]), [120, 60, 40, 30, 24]);
/// ```
pub fn products(numbers: &[u32]) -> Vec<u32> {
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
/// use p2::products_without_div;
/// assert_eq!(products_without_div(&[1, 2, 3, 4, 5]), [120, 60, 40, 30, 24]);
/// ```
pub fn products_without_div(numbers: &[u32]) -> Vec<u32> {
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
    use products;
    use products_without_div;

    #[test]
    fn test_five() {
        let input = [1, 2, 3, 4, 5];
        let output = [120, 60, 40, 30, 24];
        assert_eq!(products(&input), output);
    }

    #[test]
    fn test_five_without_div() {
        let input = [1, 2, 3, 4, 5];
        let output = [120, 60, 40, 30, 24];
        assert_eq!(products_without_div(&input), output);
    }

    #[test]
    fn test_three() {
        let input = [3, 2, 1];
        let output = [2, 3, 6];
        assert_eq!(products(&input), output);
    }

    #[test]
    fn test_three_without_div() {
        let input = [3, 2, 1];
        let output = [2, 3, 6];
        assert_eq!(products_without_div(&input), output);
    }

}
