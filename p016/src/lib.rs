/*
This problem was asked by Twitter. 
 
You run an e-commerce website and want to record the last N order ids in a log. 
Implement a data structure to accomplish this, with the following API: 
 
 * record(order_id): adds the order_id to the log 
 * get_last(i): gets the ith last element from the log. i is guaranteed to be 
 smaller than or equal to N. 
 
You should be as efficient with time and space as possible. 
*/

// This sounds like a circular buffer. We can implement this with a Vec

#[derive(Debug)]
struct OrderLog<'a> {
    orders: Vec<Option<&'a str>>,
    first: usize,
    len: usize,
}

impl<'a> OrderLog<'a> {
    fn new(capacity: usize) -> Self {
        OrderLog {
            orders: vec![None; capacity],
            first: 0,
            len: 0,
        }
    }

    fn capacity(&self) -> usize {
        self.orders.len()
    }

    fn is_full(&self) -> bool {
        self.len == self.capacity()
    }

    fn record(&mut self, order_id: &'a str) {
        let capacity = self.capacity();
        if self.is_full() {
            self.orders[self.first] = Some(order_id);
            self.first = (self.first + 1) % capacity;
        } else {
            self.orders[(self.first + self.len) % capacity] = Some(order_id);
            self.len += 1;
        }
    }

    fn get_last(&self, i: usize) -> Option<&str> {
        assert!(i < self.capacity());
        if i >= self.len {
            return None;
        }
        // Invert position for easier indexing.
        let j = self.len - i - 1;
        self.orders[(self.first + j) % self.capacity()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_order_log() {
        let mut log: OrderLog = OrderLog::new(3);
        assert_eq!(log.get_last(0), None);
        log.record("1");
        assert_eq!(log.get_last(0), Some("1"));
        log.record("2");
        assert_eq!(log.get_last(0), Some("2"));
        assert_eq!(log.get_last(1), Some("1"));
        log.record("3");
        assert_eq!(log.get_last(0), Some("3"));
        assert_eq!(log.get_last(1), Some("2"));
        assert_eq!(log.get_last(2), Some("1"));
        log.record("4");
        assert_eq!(log.get_last(0), Some("4"));
        assert_eq!(log.get_last(1), Some("3"));
        assert_eq!(log.get_last(2), Some("2"));
    }
}
