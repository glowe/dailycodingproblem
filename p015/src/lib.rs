/*
Good morning! Here's your coding interview problem for today. 
 
This problem was asked by Facebook. 
 
Given a stream of elements too large to store in memory, pick a random element 
from the stream with uniform probability. 
*/
extern crate rand;
use rand::{thread_rng, Rng};

fn random_choice<I, T>(iter: &mut I) -> Option<T>
where
    I: Iterator<Item = T>,
{
    let mut rng = thread_rng();
    let mut choice = None;
    for (i, e) in iter.enumerate() {
        if i == 0 {
            choice = Some(e)
        } else if rng.gen_range(1, i + 1) == 1 {
            choice = Some(e)
        }
    }
    choice
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_random_choice() {
        extern crate kolmogorov_smirnov as ks;
        let stream = (0..1_000).collect::<Vec<u64>>();
        let choices = stream
            .clone()
            .iter()
            .map(|_| random_choice(&mut stream.clone().into_iter()).unwrap())
            .collect::<Vec<u64>>();
        let result = ks::test(&stream, &choices, 0.95);
        assert!(!result.is_rejected);
    }
}
