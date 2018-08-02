fn car(pair: Box<Fn(&Fn(u32, u32) -> u32) -> u32>) -> u32 {
    pair(&|a, _| a)
}

fn cdr(pair: Box<Fn(&Fn(u32, u32) -> u32) -> u32>) -> u32 {
    pair(&|_, b| b)
}

fn cons(x: u32, y: u32) -> Box<Fn(&Fn(u32, u32) -> u32) -> u32> {
    Box::new(move |func| func(x, y))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_car() {
        assert_eq!(car(cons(3, 4)), 3);
    }

    #[test]
    fn test_cdr() {
        assert_eq!(cdr(cons(3, 4)), 4);
    }
}
