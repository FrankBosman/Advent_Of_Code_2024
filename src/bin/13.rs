
advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<u64> {
    solve_puzzle(input, false)
}

pub fn part_two(input: &str) -> Option<u64> {
    solve_puzzle(input, true)
}

fn solve_puzzle(input: &str, part2: bool) -> Option<u64> {
    let machines: Vec<Machine> = input.split("\n\n").map(|machine_str| Machine::new(machine_str)).collect();
    let mut sum = 0;
    for machine in machines {
        if let Some(x) = machine.calc_tokens(part2) {
            sum += x;
        }
    }
    Some(sum)
}

#[derive(Debug)]
struct Machine {
    a: [i64; 2],
    b: [i64; 2],
    c: [i64; 2],
}

impl Machine {
    fn new(input: &str) -> Machine {
        let lines = input.lines();

        let mut params = [[0; 2]; 3];
        for (i, line) in lines.enumerate() {
            params[i] = parse_line(line);
        }

        Self { a: params[0], b: params[1], c: params[2] }
    }

    fn calc_tokens(&self, part2: bool) -> Option<u64> {
        let result = self.solve(part2);
        if result.is_none() { return None; }
        let (a, b) = result.unwrap();

        if !part2 && (a > 100 || b > 100) { None } else { Some(a * 3 + b) }
    }

    fn solve(&self, part2: bool) -> Option<(u64, u64)> {
        let determinant = self.determinant();

        // Check if there exists a unique answer
        if determinant == 0 {
            println!("No unique answer available, because the determinant is 0");
            return None;
        }
        let det_a = self.determinant_of('a', part2);
        let det_b = self.determinant_of('b', part2);

        // Ensure the determinants are divisible
        if det_a % determinant != 0 || det_b % determinant != 0 { return None; }

        let a = det_a / determinant;
        let b = det_b / determinant;

        if a < 0 || b < 0 { return None; }
        Some((a as u64, b as u64))
    }

    fn determinant(&self) -> i64 {
        self.a[0] * self.b[1] - self.a[1] * self.b[0]
    }
    fn determinant_of(&self, of: char, part2: bool) -> i64 {
        // increment by 10000000000000 for part 2
        let mut c = self.c;
        if part2 {
            c[0] += 10000000000000;
            c[1] += 10000000000000;
        }
        match of {
            'a' => c[0] * self.b[1] - c[1] * self.b[0],
            'b' => self.a[0] * c[1] - self.a[1] * c[0],
            _ => panic!("Invalid determinant of, choose 'a' or 'b'")
        }
    }
}

fn parse_line(line: &str) -> [i64;2] {
    let mut temp = 0i64;
    let mut completed = [0i64;2];
    for char in line.chars() {
        if char.is_ascii_digit() {
            temp = temp * 10 + char.to_digit(10).unwrap() as i64;
        } else if temp > 0 {
            completed[0] = temp;
            temp = 0;
        }
    }
    completed[1] = temp;

    completed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
