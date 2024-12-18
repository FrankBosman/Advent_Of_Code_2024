use advent_of_code::helpers::parse_to;

advent_of_code::solution!(17);

pub fn part_one(input: &str) -> Option<String> {
    let (mut computer, program) = process_input(input);

    // run the program, and parse the result
    computer.run(&program, None);
    Some(computer.output.iter().map(|val| val.to_string()).collect::<Vec<_>>().join(","))
}

pub fn part_two(input: &str) -> Option<u64> {
    let (computer, program) = process_input(input);

    Some(back_propagate(0, 0, computer, &program))
}

fn back_propagate(register_a: u64, completed_out: usize, mut computer: Computer, program: &Vec<u64>) -> u64 {
    // Every step of the program looks at the least significant bits of A, and then removes them.
    // Then to back propagate we have to shift the registerA bits by 3 (multiply by 8) and then loop over all possibilities and check them individually
    // the possibilities are three bits, 2^3 = 0-8 (excl.)
    let mut minimum = u64::MAX;
    for offset in 0..8 {
        let next_a = (register_a << 3) + offset;
        computer.run(program, Some(next_a));

        // Test if the output is the same as the program output for completed_out number of elements
        if computer.output.eq(&program[program.len() - completed_out - 1..]) {
            // Check if we are already done
            if computer.output.eq(program) { return next_a; }

            // Continue testing for this register A
            let result = back_propagate(next_a, completed_out + 1, Computer::from(&computer), program);
            if result < minimum { minimum = result }
        }
    }

    minimum
}

fn process_input(input: &str) -> (Computer, Vec<u64>) {
    let (register_str, program_str) = input.split_once("\n\n").unwrap();
    let computer = Computer::new(register_str);
    let program = program_str.replace("Program: ", "")
        .trim().split(',').map(|str| str.parse::<u64>().unwrap()).collect::<Vec<_>>();

    (computer, program)
}

#[derive(Debug)]
struct Computer {
    registers: [u64; 3],
    output: Vec<u64>,
}

impl Computer {
    pub(crate) fn from(computer: &Computer) -> Computer {
        Computer {
            registers: [computer.registers[0], 0, 0],
            output: Vec::new(),
        }
    }
    pub(crate) fn new(register_str: &str) -> Self {
        let params = parse_to::extract_numbers(register_str);
        Computer {
            registers: [params[0] as u64, params[1] as u64, params[2] as u64],
            output: Vec::new(),
        }
    }

    pub(crate) fn process(&mut self, opcode: u64, operator: u64, instruction_pointer: usize) -> usize {
        match opcode {
            0 | 6 | 7 => {  // adv
                let parameter = self.combo_parameter(operator);
                let location = match opcode {
                    6 => 1,
                    7 => 2,
                    _ => 0,
                };
                self.registers[location] = self.registers[0] / 2u64.pow(parameter as u32);
            }
            1 => {  // bxl
                self.registers[1] ^= operator;
            }
            2 => {  // bst
                let parameter = self.combo_parameter(operator);
                self.registers[1] = parameter % 8u64;
            }
            3 => {  // jnz
                if self.registers[0] != 0 { return operator as usize; }
            }
            4 => {  // bxc
                self.registers[1] ^= self.registers[2];
            }
            5 => {  // out
                let parameter = self.combo_parameter(operator);
                self.output.push(parameter % 8u64);
            }
            _ => panic!("Invalid opcode: {opcode}"),
        }

        instruction_pointer + 2
    }

    pub(crate) fn run(&mut self, program: &Vec<u64>, register_a: Option<u64>) {
        // Reset the computer if a new value for register A is given
        if let Some(register_a) = register_a {
            self.reset(register_a);
        }

        // Keep track of the current action, and when it is finished
        let mut instruction_pointer = 0;
        let program_len = program.len();

        while instruction_pointer < program_len - 1 {
            let (opcode, operand) = (program[instruction_pointer], program[instruction_pointer + 1]);
            instruction_pointer = self.process(opcode, operand, instruction_pointer);
        }
    }

    pub(crate) fn reset(&mut self, register_a: u64) {
        self.registers.fill(0);
        self.registers[0] = register_a;
        self.output.clear();
    }

    fn combo_parameter(&self, operator: u64) -> u64 {
        // Combo operands 0 through 3 represent literal values 0 through 3.
        // Combo operand 4 represents the value of register A.
        // Combo operand 5 represents the value of register B.
        // Combo operand 6 represents the value of register C.
        // Combo operand 7 is reserved and will not appear in valid programs.

        if operator == 7 { panic!("Operator can't be 7 if it is an combo operator") };
        if operator < 4 { operator } else { self.registers[(operator - 4) as usize] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("5,7,3,0".into()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(117440));
    }
}


/*

Operations:
2,4: B = A % 8
1,1: B = B ^ 1
7,5: C = A / 2^B
1,5: B = B ^ 5
4,0: B = B ^ C
5,5; Out: B % 8
3,0:

out1 needs to be 2, so
B % 8 = 2
B = x * 8 + 2

B ^ C = B
B = B ^ C

B ^ 5 = B
B = B ^ 5

A / 2 ^ B = C




2,4: A: 64854237, B: 5, C: 0
1,1: A: 64854237, B: 4, C: 0
7,5: A: 64854237, B: 4, C: 4053389
1,5: A: 64854237, B: 1, C: 4053389
1,5: A: 64854237, B: 4053388, C: 4053389
5,5: Out 4

2,4:
Register A: 64854237
Register B: 5
Register C: 0

1,1:
Register A: 64854237
Register B: 4
Register C: 0

7,5:
Register A: 64854237
Register B: 4
Register C: 4053389

1,5:
Register A: 64854237
Register B: 0
Register C: 4053389

4,0:
Register A: 64854237
Register B: 4053389
Register C: 4053389

5,5
Register A: 64854237
Register B: 4053389
Register C: 4053389
output 5

0,3

 */
