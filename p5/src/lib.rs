fn car<T: Copy>(pair: Box<Fn(&Fn(T, T) -> T) -> T>) -> T {
    pair(&|a, _| a)
}

fn cdr<T: Copy>(pair: Box<Fn(&Fn(T, T) -> T) -> T>) -> T {
    pair(&|_, b| b)
}

fn cons<T: 'static>(x: T, y: T) -> Box<Fn(&Fn(T, T) -> T) -> T>
where
    T: Copy,
{
    Box::new(move |func| func(x, y))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_car() {
        assert_eq!(car(cons(3, 4)), 3);
        assert_eq!(car(cons("car", "cdr")), "car");
    }

    #[test]
    fn test_cdr() {
        assert_eq!(cdr(cons(3, 4)), 4);
        assert_eq!(cdr(cons("car", "cdr")), "cdr");
    }
}
