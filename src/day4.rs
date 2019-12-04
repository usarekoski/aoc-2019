pub fn solve1() -> u64 {
    let input_low = 178416;
    let input_high = 676461;

    let mut count = 0;
    for i in input_low..=input_high {
        let digits = i.to_string();
        let two_adjacent_digits = digits
            .chars()
            .zip(digits.chars().skip(1))
            .any(|(a, b)| a == b);
        let digits_never_decreases = digits
            .chars()
            .zip(digits.chars().skip(1))
            .all(|(a, b)| a <= b);

        if two_adjacent_digits && digits_never_decreases {
            count += 1;
        }
    }

    count
}

pub fn solve2() -> u64 {
    let input_low = 178416;
    let input_high = 676461;

    let mut count = 0;
    for i in input_low..=input_high {
        let digits = i.to_string();
        let mut two_adjacent_digits = digits
            .chars()
            .zip(digits.chars().skip(1))
            .zip(digits.chars().skip(2))
            .zip(digits.chars().skip(3))
            .any(|(((a, b), c), d)| (b == c && b != a && b != d));

        // Check adjacent digits at begin and end:
        let digits_bytes = digits.as_bytes();
        let last = digits_bytes.len() - 1;
        if digits_bytes[0] == digits_bytes[1] && digits_bytes[2] != digits_bytes[0]
            || digits_bytes[last] == digits_bytes[last - 1]
                && digits_bytes[last - 2] != digits_bytes[last]
        {
            two_adjacent_digits = true;
        }

        let digits_never_decreases = digits
            .chars()
            .zip(digits.chars().skip(1))
            .all(|(a, b)| a <= b);

        if two_adjacent_digits && digits_never_decreases {
            count += 1;
        }
    }

    count
}
