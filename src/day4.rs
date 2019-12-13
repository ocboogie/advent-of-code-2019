use std::ops::Range;

fn has_double(num: &str) -> bool {
    let digits = num.chars().collect::<Vec<_>>();

    for (i, digit) in digits.iter().enumerate() {
        if let Some(next_digit) = digits.get(i + 1) {
            if digit == next_digit {
                return true;
            }
        }
    }
    false
}

// Super inefficient but gets the job done :)
fn just_double(num: &str) -> bool {
    let mut digits_in_row: Vec<(char, u32)> = Vec::new();

    for digit in num.chars() {
        if let Some((_, in_a_row)) = digits_in_row
            .last_mut()
            .filter(|(row_digit, _)| *row_digit == digit)
        {
            *in_a_row += 1;
        } else {
            digits_in_row.push((digit, 1));
        }
    }
    digits_in_row.iter().any(|(_, row)| *row == 2)
}

// Ah yes this is much better
fn just_double_v2(num: &str) -> bool {
    let mut current_digit = 'z';
    let mut in_a_row = 0;

    for digit in num.chars() {
        if digit == current_digit {
            in_a_row += 1;
        } else {
            if in_a_row == 2 {
                return true;
            }
            current_digit = digit;
            in_a_row = 1;
        }
    }
    in_a_row == 2
}

fn never_decreases(num: &str) -> bool {
    let mut max = None;

    for digit in num.chars() {
        if let Some(max_num) = max {
            if (digit as u8) < max_num {
                return false;
            }
        }
        max = Some(digit as u8)
    }

    true
}

fn meets(num: u32) -> bool {
    let string = num.to_string();
    return string.len() == 6 && has_double(&string) && never_decreases(&string);
}

fn meets_two(num: u32) -> bool {
    let string = num.to_string();
    return string.len() == 6 && just_double_v2(&string) && never_decreases(&string);
}

#[aoc_generator(day4)]
fn generator_input(input: &str) -> Range<u32> {
    let mut nums = input.split("-").filter_map(|num| num.parse::<u32>().ok());

    nums.next().unwrap()..nums.next().unwrap()
}

#[aoc(day4, part1)]
fn part_one(range: &Range<u32>) -> u32 {
    let mut count = 0;

    for i in range.clone() {
        if meets(i) {
            count += 1;
        }
    }

    return count;
}

#[aoc(day4, part2)]
fn part_two(range: &Range<u32>) -> u32 {
    let mut count = 0;

    for i in range.clone() {
        if meets_two(i) {
            count += 1;
        }
    }

    return count;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_has_double() {
        assert!(has_double("oiea11nrst"));
        assert!(!has_double("8912389"));
        assert!(has_double("1187789271"));
        assert!(has_double("980923855"));
    }

    #[test]
    fn test_never_decrease() {
        assert!(never_decreases("111111"));
        assert!(never_decreases("222222"));
        assert!(!never_decreases("123454"));
        assert!(!never_decreases("012343"));
        assert!(never_decreases("01234"));
    }

    #[test]
    fn test_part_one() {
        assert!(meets(111111));
        assert!(!meets(223450));
        assert!(!meets(123789));
    }

    #[test]
    fn test_part_two() {
        assert!(meets_two(112233));
        assert!(!meets_two(123444));
        assert!(meets_two(111122));
    }
}
