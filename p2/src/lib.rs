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

/// Computes a Vec of products.
///
/// Each element of the Vec is the product of all the elements in the slice
/// except for the element at the corresponding position.
///
/// Examples:
/// ```
/// use p2::products_without_division_linear;
/// assert_eq!(products_without_division_linear(&[1, 2, 3, 4, 5]), [120, 60, 40, 30, 24]);
/// ```
pub fn products_without_division_linear(numbers: &[u32]) -> Vec<u32> {
    let mut products = Vec::with_capacity(numbers.len());
    let mut product = 1;
    products.push(product);
    // first pass: fill with preceeding products
    for i in 1..numbers.len() {
        product *= numbers[i - 1];
        products.push(product);
    }
    // second pass: fill with subsequent products
    product = 1;
    for i in (0..numbers.len() - 1).rev() {
        product *= numbers[i + 1];
        products[i] *= product;
    }
    products
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

    #[test]
    fn test_without_division_linear() {
        use products_without_division_linear;
        assert_eq!(
            products_without_division_linear(&[1, 2, 3, 4, 5]),
            [120, 60, 40, 30, 24]
        );
        assert_eq!(products_without_division_linear(&[3, 2, 1]), [2, 3, 6]);
    }
}
