fn main() {
    let input = include_str!("./input.txt");
    let output_1 = part_1(input);
    println!("part 1: {}", output_1);
    let output_2 = part_2(input);
    println!("part 2: {}", output_2);
}

fn part_1(input: &str) -> String {
    let mut sum = 0;
    for line in input.to_string().lines() {
        sum += scan_for_digit(line.chars()) * 10 + scan_for_digit(line.chars().rev());
    }

    format!("{}", sum)
}

struct CharMachine<'a> {
    num_str: &'a str,
    num_u32: u32,
    pos: usize,
    length: usize,
}

impl<'a> CharMachine<'a> {
    fn new(num_str: &'a str, num_u32: u32) -> CharMachine<'a> {
        CharMachine {
            num_str: &num_str,
            num_u32,
            pos: 0,
            length: num_str.len(),
        }
    }
    fn check(&mut self, c: char, reverse: bool) -> bool {
        let char;
        let first_num_char;
        if reverse {
            char = self
                .num_str
                .chars()
                .rev()
                .nth(self.pos)
                .expect("pos can't be > length");
            first_num_char = self
                .num_str
                .chars()
                .last()
                .expect("should always be at least one char");
        } else {
            char = self
                .num_str
                .chars()
                .nth(self.pos)
                .expect("pos can't be > length");
            first_num_char = self
                .num_str
                .chars()
                .next()
                .expect("should always be at least one char");
        }
        if char == c {
            self.pos += 1;
            return self.pos == self.length;
        } else if c == first_num_char {
            self.pos = 1;
            return false;
        } else {
            self.pos = 0;
            return false;
        }
    }
}

fn scan_for_digit(line: impl Iterator<Item = char>) -> u32 {
    for c in line {
        if let Some(n) = c.to_digit(10) {
            return n;
        }
    }
    0
}

fn scan_for_digit_with_machines(line: impl Iterator<Item = char>, reverse: bool) -> u32 {
    let mut machines = vec![
        CharMachine::new("one", 1),
        CharMachine::new("two", 2),
        CharMachine::new("three", 3),
        CharMachine::new("four", 4),
        CharMachine::new("five", 5),
        CharMachine::new("six", 6),
        CharMachine::new("seven", 7),
        CharMachine::new("eight", 8),
        CharMachine::new("nine", 9),
    ];
    for c in line {
        for m in machines.iter_mut() {
            if m.check(c, reverse) {
                return m.num_u32;
            }
        }
        if let Some(n) = c.to_digit(10) {
            return n;
        }
    }
    0
}

fn part_2(input: &str) -> String {
    let mut sum = 0;
    for line in input.to_string().lines() {
        let tens = scan_for_digit_with_machines(line.chars(), false);
        let ones = scan_for_digit_with_machines(line.chars().rev(), true);
        let calibration = tens * 10 + ones;
        sum += calibration;
    }

    format!("{}", sum)
}

#[test]
fn test_shared_letter() {
    let line = "dtkjdncq73threechgcccdgqqsixthreehlfroneightn";
    assert!(scan_for_digit_with_machines(line.chars(), false) == 7);
    assert!(scan_for_digit_with_machines(line.chars().rev(), true) == 8);
}

#[test]
fn test_double_letter() {
    let line = "xcsmcfour3eightts";
    assert!(scan_for_digit_with_machines(line.chars(), false) == 4);
    assert!(scan_for_digit_with_machines(line.chars().rev(), true) == 8);
}
