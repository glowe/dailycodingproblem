/*
Good morning! Here's your coding interview problem for today.

This problem was asked by Google.

The area of a circle is defined as πr^2. Estimate π to 3 decimal places using a Monte Carlo method.

Hint: The basic equation of a circle is x2 + y2 = r2.
*/
#![feature(non_ascii_idents)]
extern crate rand;
extern crate rayon;
use rand::{thread_rng, Rng};
use rayon::prelude::*;

fn estimate_π(iterations: u64) -> f64 {
    let points_in_square = iterations as f64;
    let points_in_circle: f64 = (0..iterations)
        .into_par_iter()
        .filter_map(|_| {
            let mut rng = thread_rng();
            let x = rng.gen::<f64>();
            let y = rng.gen::<f64>();
            let dist = (x * x + y * y).sqrt();
            if dist <= 1.0 {
                Some(1.0)
            } else {
                None
            }
        }).sum();
    4.0 * points_in_circle as f64 / points_in_square
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_estimate_π() {
        let π = estimate_π(10_000_000);
        let actual = format!("{:.3}", π);
        assert_eq!(actual, "3.141");
    }
}
