use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut diffs: HashMap<i32, usize> = HashMap::new();
    let mut result: Vec<i32> = vec![];
    for (index, val) in nums.iter().enumerate() {
        let diff = target - *val;
        let diff_from_hashmap = diffs.get(val);
        if let Some(idx) = diff_from_hashmap {
            result.push(*idx as i32);
            result.push(index as i32);
            break;
        } else {
            diffs.insert(diff, index);
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(vec![0, 1], two_sum(vec![2, 7, 11, 15], 9));
    }

    #[test]
    fn example_2() {
        assert_eq!(vec![1, 2], two_sum(vec![3, 2, 4], 6));
    }

    #[test]
    fn example_3() {
        assert_eq!(vec![0, 1], two_sum(vec![3, 3], 6));
    }
}
