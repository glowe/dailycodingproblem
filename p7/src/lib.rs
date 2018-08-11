/// Good morning! Here's your coding interview problem for today.
/// This problem was asked by Facebook.
/// Given the mapping a = 1, b = 2, ... z = 26, and an encoded message, count the
/// number of ways it can be decoded.
/// For example, the message '111' would give 3, since it could be decoded as 'aaa',
/// 'ka', and 'ak'.
/// You can assume that the messages are decodable. For example, '001' is not
/// allowed.

fn can_decode(message: &str) -> bool {
    match message.parse::<u64>() {
        Err(_) => false,
        Ok(number) => 1 <= number && number <= 26,
    }
}

pub fn count_decodings(message: &str) -> u64 {
    if message.len() == 0 {
        return 0;
    }

    if message.len() == 1 {
        return if can_decode(message) { 1 } else { 0 };
    }

    if message.len() == 2 {
        let two_char_decodings = if can_decode(message) { 1 } else { 0 };
        let one_char_slices_can_decode = can_decode(&message[0..1]) && can_decode(&message[1..2]);
        return two_char_decodings + if one_char_slices_can_decode { 1 } else { 0 };
    }

    let first = &message[..1];
    if !can_decode(first) {
        // If the first character doesn't decode, then we don't have a valid decoding.
        return 0;
    }

    let mut decodings = 0;
    let rest = &message[1..];
    decodings += count_decodings(rest);

    let first_two = &message[..2];
    if !can_decode(first_two) {
        return decodings;
    }

    let rest = &message[2..];
    if rest.len() == 0 {
        return decodings;
    }

    decodings + count_decodings(&rest)
}

#[cfg(test)]
mod tests {
    use super::count_decodings;

    #[test]
    fn test_count_decodings_6() {
        assert_eq!(count_decodings("1221"), 5);
    }

    #[test]
    fn test_count_decodings_3() {
        assert_eq!(count_decodings("111"), 3);
        assert_eq!(count_decodings("126"), 3);
    }

    #[test]
    fn test_count_decodings_2() {
        assert_eq!(count_decodings("26"), 2);
        assert_eq!(count_decodings("127"), 2);
    }

    #[test]
    fn test_count_decodings_1() {
        assert_eq!(count_decodings("271"), 1);
        assert_eq!(count_decodings("27"), 1);
        assert_eq!(count_decodings("7"), 1);
    }

    #[test]
    fn test_count_decodings_0() {
        assert_eq!(count_decodings("100"), 0);
        assert_eq!(count_decodings("0"), 0);
        assert_eq!(count_decodings(""), 0);
        assert_eq!(count_decodings("a"), 0);
    }
}
