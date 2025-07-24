fn length_of_longest_substring(s: String) -> i32 {
    if s.is_empty() {
        return 0;
    }

    let mut map: [bool; 128] = [false; 128];
    let mut longest = 0i32;
    let mut length = 0i32;
    let mut behind = 0usize;
    let mut current = 0usize;
    let input = s.as_bytes();
    let input_len = input.len();

    while current < input_len {
        if !map[input[current] as usize] {
            map[input[current] as usize] = true;
            length += 1;
        } else {
            (longest < length).then(|| longest = length);
            while input[behind] != input[current] {
                map[input[behind] as usize] = false;
                behind += 1;
                length -= 1;
            }
            behind += 1;
        }
        current += 1;
    }
    (longest < length).then(|| longest = length);

    longest
}
