use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Instruction {
    Right(u32),
    Down(u32),
    Left(u32),
    Up(u32),
}

impl Instruction {
    fn dist(&self) -> u32 {
        match self {
            Instruction::Right(dist) => *dist,
            Instruction::Down(dist) => *dist,
            Instruction::Left(dist) => *dist,
            Instruction::Up(dist) => *dist,
        }
    }

    fn from_str(input: &str) -> Self {
        let mut chars = input.chars();
        let dir = chars.next().unwrap();
        let amount = chars.collect::<String>().parse::<u32>().unwrap();

        use Instruction::*;
        match dir {
            'R' => Right(amount),
            'D' => Down(amount),
            'L' => Left(amount),
            'U' => Up(amount),
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug)]
struct Intersection {
    x: i32,
    y: i32,
    steps: u32,
}

struct Wire(HashMap<(i32, i32), u32>);

impl Wire {
    fn new(instructions: &[Instruction]) -> Self {
        let mut x = 0;
        let mut y = 0;
        let mut steps = 0;
        let mut map = HashMap::new();

        for instruction in instructions {
            for _ in 0..instruction.dist() {
                match instruction {
                    Instruction::Right(_) => x += 1,
                    Instruction::Down(_) => y += 1,
                    Instruction::Left(_) => x -= 1,
                    Instruction::Up(_) => y -= 1,
                }
                steps += 1;
                map.insert((x, y), steps);
            }
        }

        Self(map)
    }

    fn intersect<'a>(&'a self, other: &'a Wire) -> impl Iterator<Item = Intersection> + 'a {
        self.0.iter().filter_map(move |(pos, steps)| {
            Some(Intersection {
                steps: other.0.get(pos)? + steps,
                x: pos.0,
                y: pos.1,
            })
        })
    }
}

#[aoc_generator(day3)]
fn generator_input(input: &str) -> Vec<Vec<Instruction>> {
    input
        .lines()
        .map(|a| a.split(",").map(Instruction::from_str).collect())
        .collect()
}

#[aoc(day3, part1)]
fn part_one(input: &[Vec<Instruction>]) -> u32 {
    let mut wires = input.iter().map(|a| Wire::new(a));
    let first_wire = wires.next().unwrap();
    let second_wire = wires.next().unwrap();

    first_wire
        .intersect(&second_wire)
        .map(|intersection| (intersection.x.abs() + intersection.y.abs()) as u32)
        .min()
        .unwrap()
}

#[aoc(day3, part2)]
fn part_two(input: &[Vec<Instruction>]) -> u32 {
    let mut wires = input.iter().map(|a| Wire::new(a));
    let first_wire = wires.next().unwrap();
    let second_wire = wires.next().unwrap();

    first_wire
        .intersect(&second_wire)
        .map(|intersection| intersection.steps)
        .min()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one(&generator_input(
                "R8,U5,L5,D3
U7,R6,D4,L4",
            )),
            6
        );
        assert_eq!(
            part_one(&generator_input(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83",
            )),
            159
        );
        assert_eq!(
            part_one(&generator_input(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
            )),
            135
        );
    }
    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two(&generator_input(
                "R8,U5,L5,D3
U7,R6,D4,L4",
            )),
            30
        );
        assert_eq!(
            part_two(&generator_input(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83",
            )),
            610
        );
        assert_eq!(
            part_two(&generator_input(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
            )),
            410
        );
    }
}
