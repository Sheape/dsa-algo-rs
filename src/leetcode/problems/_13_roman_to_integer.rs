fn roman_to_int(s: String) -> i32 {
    let mut index = 0usize;
    let mut result = 0i32;
    let input = s.as_bytes();

    while index < input.len() {
        let (res_added, should_fwd_index) = match input[index] {
            b'I' => match input.get(index + 1) {
                Some(b'V') => (4, true),
                Some(b'X') => (9, true),
                _ => (1, false),
            },
            b'V' => (5, false),
            b'X' => match input.get(index + 1) {
                Some(b'L') => (40, true),
                Some(b'C') => (90, true),
                _ => (10, false),
            },
            b'L' => (50, false),
            b'C' => match input.get(index + 1) {
                Some(b'D') => (400, true),
                Some(b'M') => (900, true),
                _ => (100, false),
            },
            b'D' => (500, false),
            b'M' => (1000, false),
            _ => (0, false),
        };

        result += res_added;

        if should_fwd_index {
            index += 2;
        } else {
            index += 1;
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(3, roman_to_int("III".into()))
    }

    #[test]
    fn example_2() {
        assert_eq!(58, roman_to_int("LVIII".into()))
    }

    #[test]
    fn example_3() {
        assert_eq!(1994, roman_to_int("MCMXCIV".into()))
    }
}
