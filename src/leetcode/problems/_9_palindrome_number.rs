fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }

    if x < 10 {
        return true;
    }

    if x < 100 {
        return x % 10 == x / 10;
    }

    let digits = x.ilog10() + 1;

    let mut is_palindrome = true;

    let mut left_partition = x;
    let mut right_partition = x;

    for i in 0..(digits / 2) {
        let left: i32 = left_partition / 10i32.pow(digits - i - 1);
        let right = right_partition % 10;

        if left != right {
            is_palindrome = false;
            break;
        }

        left_partition %= 10i32.pow(digits - i - 1);
        right_partition /= 10;
    }

    is_palindrome
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn example_1() {
        assert!(is_palindrome(121))
    }

    #[test]
    fn example_2() {
        assert!(!is_palindrome(-121))
    }

    #[test]
    fn example_3() {
        assert!(!is_palindrome(10))
    }
}
