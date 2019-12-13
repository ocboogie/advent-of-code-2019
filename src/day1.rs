#[aoc_generator(day1)]
fn generator_input(input: &str) -> Vec<i32> {
    input.lines().map(|a| a.parse::<i32>().unwrap()).collect()
}

/// Calculates the fuel necessary for a given mass
fn calc_fuel(x: i32) -> i32 {
    x / 3 - 2
}

fn calc_total_fuel(mass: i32) -> i32 {
    let fuel = ((mass as f32 / 3.0).trunc() - 2.0) as i32;
    if fuel <= 0 {
        return 0;
    }

    fuel + calc_total_fuel(fuel)
}

#[aoc(day1, part1)]
fn part_one(input: &[i32]) -> i32 {
    input.iter().map(|i| calc_fuel(*i)).sum()
}

#[aoc(day1, part2)]
fn part_two(input: &[i32]) -> i32 {
    input.iter().map(|i| calc_total_fuel(*i)).sum()
}

#[cfg(test)]
pub mod tests {
    use super::{calc_fuel, calc_total_fuel};

    #[test]
    fn day1_part1() {
        assert_eq!(calc_fuel(12), 2);
        assert_eq!(calc_fuel(14), 2);
        assert_eq!(calc_fuel(1969), 654);
        assert_eq!(calc_fuel(100756), 33583);
    }

    #[test]
    fn day1_part2() {
        assert_eq!(calc_total_fuel(14), 2);
        assert_eq!(calc_total_fuel(1969), 966);
        assert_eq!(calc_total_fuel(100756), 50346);
    }
}
