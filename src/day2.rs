#[aoc_generator(day2)]
fn generator_input(input: &str) -> Vec<usize> {
    input
        .split(",")
        .map(|a| a.parse::<usize>().unwrap())
        .collect()
}

struct Runtime {
    pub memory: Vec<usize>,
    pub ip: usize,
}

impl Runtime {
    fn new(memory: Vec<usize>) -> Self {
        Self { memory, ip: 0 }
    }

    fn run(&mut self) {
        loop {
            match self.memory[self.ip] {
                1 => {
                    let first = self.memory[self.memory[self.ip + 1]];
                    let second = self.memory[self.memory[self.ip + 2]];
                    let positon = self.memory[self.ip + 3];

                    self.memory[positon] = first + second;
                    self.ip += 4;
                }
                2 => {
                    let first = self.memory[self.memory[self.ip + 1]];
                    let second = self.memory[self.memory[self.ip + 2]];
                    let positon = self.memory[self.ip + 3];

                    self.memory[positon] = first * second;
                    self.ip += 4;
                }
                99 => {
                    break;
                }
                opcode => {
                    panic!("Tried to run unknown opcode: {}", opcode);
                }
            }
        }
    }
}

#[aoc(day2, part1)]
fn part_one(input: &[usize]) -> usize {
    let mut memory = input.to_vec();

    memory[1] = 12;
    memory[2] = 2;
    let mut rt = Runtime::new(memory);
    rt.run();
    return rt.memory[0];
}

#[aoc(day2, part2)]
fn part_two(input: &[usize]) -> usize {
    let memory = input.to_vec();

    for noun in 0..99 {
        for verb in 0..99 {
            let mut temp_memory = memory.clone();
            temp_memory[1] = noun;
            temp_memory[2] = verb;
            let mut rt = Runtime::new(temp_memory);
            rt.run();

            if rt.memory[0] == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    0
}

#[cfg(test)]
pub mod tests {
    use super::Runtime;

    #[test]
    fn day1() {
        let mut rt = Runtime::new(vec![1, 0, 0, 0, 99]);
        rt.run();
        assert_eq!(rt.memory, vec![2, 0, 0, 0, 99]);

        rt = Runtime::new(vec![2, 3, 0, 3, 99]);
        rt.run();
        assert_eq!(rt.memory, vec![2, 3, 0, 6, 99]);

        rt = Runtime::new(vec![2, 4, 4, 5, 99, 0]);
        rt.run();
        assert_eq!(rt.memory, vec![2, 4, 4, 5, 99, 9801]);

        rt = Runtime::new(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
        rt.run();
        assert_eq!(rt.memory, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
