fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut unique = 1usize;
    let mut cur_val = nums[0];
    for i in 1..nums.len() {
        if nums[i] != cur_val {
            nums[unique] = nums[i];
            cur_val = nums[unique];
            unique += 1;
        }
    }

    for val in nums.iter_mut().skip(unique) {
        *val = 0;
    }

    unique as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_1() {
        let mut vec = vec![1, 1, 2];
        assert_eq!(2, remove_duplicates(&mut vec));
        assert_eq!(vec[0..2], [1, 2]);
        assert_eq!(vec[2], 0);
    }

    #[test]
    fn example_2() {
        let mut vec = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        assert_eq!(5, remove_duplicates(&mut vec));
        assert_eq!(vec[0..5], [0, 1, 2, 3, 4]);
        assert_eq!(vec[5..10], [0, 0, 0, 0, 0]);
    }
}
