/* 
Good morning! Here's your coding interview problem for today.
This problem was asked by Apple.
Implement a job scheduler which takes in a function f and an integer n, and calls f after n milliseconds.
*/
#![feature(duration_as_u128)]
use std::thread;
use std::time::{Duration, Instant};

fn schedule<F>(f: F, n: u64)
where
    F: FnOnce(),
    F: Send + 'static,
{
    let millis = Duration::from_millis(n);
    thread::spawn(move || {
        thread::sleep(millis);
        f();
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scheduler() {
        use std::sync::mpsc;
        let (tx, rx) = mpsc::channel();
        let now = Instant::now();
        let f = move || {
            let elapsed = now.elapsed().as_millis();
            tx.send(elapsed).unwrap();
        };
        schedule(f, 300);
        let elapsed = rx.recv().unwrap();
        let delta = ((elapsed - 300) as i64).abs();
        assert!(delta < 10);
    }
}
