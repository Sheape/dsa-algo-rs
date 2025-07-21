fn is_valid(s: String) -> bool {
    if s.len() == 1 {
        return false;
    }

    let mut stack: Vec<u8> = vec![];
    let input = s.as_bytes();
    let mut input_is_valid = true;

    for ch in input.iter() {
        if !input_is_valid {
            break;
        }

        match ch {
            b'(' | b'[' | b'{' => stack.push(*ch),
            b')' => {
                if stack.last() == Some(&b'(') {
                    stack.pop();
                } else {
                    input_is_valid = false;
                }
            }
            b']' => {
                if stack.last() == Some(&b'[') {
                    stack.pop();
                } else {
                    input_is_valid = false;
                }
            }
            b'}' => {
                if stack.last() == Some(&b'{') {
                    stack.pop();
                } else {
                    input_is_valid = false;
                }
            }
            _ => (),
        }
    }

    stack.is_empty() && input_is_valid
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_1() {
        assert!(is_valid("()".into()))
    }

    #[test]
    fn example_2() {
        assert!(is_valid("()[]{}".into()))
    }

    #[test]
    fn example_3() {
        assert!(!is_valid("(]".into()))
    }

    #[test]
    fn example_4() {
        assert!(is_valid("([])".into()))
    }

    #[test]
    fn example_5() {
        assert!(!is_valid("([)]".into()))
    }

    #[test]
    fn example_6() {
        assert!(!is_valid("(".into()))
    }

    #[test]
    fn example_7() {
        assert!(!is_valid("((".into()))
    }
}
