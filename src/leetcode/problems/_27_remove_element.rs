fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut count = 0;

    for i in (0..nums.len()).rev() {
        if nums[i] == val {
            let index_rev = nums.len() - 1 - count;
            nums[i] = -1;
            if i != index_rev {
                nums.swap(i, index_rev);
            }
            count += 1;
        }
    }

    (nums.len() - count) as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_1() {
        let mut vec = vec![3, 2, 2, 3];
        assert_eq!(2, remove_element(&mut vec, 3));
        assert_eq!(vec[0..2], [2, 2]);
    }

    #[test]
    fn example_2() {
        let mut vec = vec![0, 1, 2, 2, 3, 0, 4, 2];
        assert_eq!(5, remove_element(&mut vec, 2));
        assert_eq!(vec[0..5], [0, 1, 0, 4, 3]);
    }
}
