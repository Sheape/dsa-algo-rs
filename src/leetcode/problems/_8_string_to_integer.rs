fn my_atoi(s: String) -> i32 {
    let mut is_negative = false;
    let mut has_stepped_in_num = false;
    let mut result = 0i32;
    let input = s.as_bytes();
    let mut cur_idx = 0;

    while cur_idx < s.len() {
        match input[cur_idx] {
            b'-' => {
                if has_stepped_in_num {
                    break;
                } else {
                    has_stepped_in_num = true;
                }
                is_negative = true;
            }
            b'+' => {
                if has_stepped_in_num {
                    break;
                } else {
                    has_stepped_in_num = true;
                }
            }
            num @ b'0'..=b'9' => {
                (!has_stepped_in_num).then(|| has_stepped_in_num = true);
                let maybe_result = result.overflowing_mul(10);
                if maybe_result.1 {
                    if is_negative {
                        result = i32::MIN;
                    } else {
                        result = i32::MAX;
                    }
                    break;
                }
                let maybe_result = maybe_result.0.overflowing_add((num - b'0') as i32);
                if maybe_result.1 {
                    if is_negative {
                        result = i32::MIN;
                    } else {
                        result = i32::MAX;
                    }
                    break;
                }
                result = maybe_result.0;
            }
            b' ' => {
                if has_stepped_in_num {
                    break;
                }
            }
            _ => {
                break;
            }
        }
        cur_idx += 1;
    }

    if is_negative {
        result *= -1;
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(my_atoi("   -042".to_owned()), -42);
    }
}
