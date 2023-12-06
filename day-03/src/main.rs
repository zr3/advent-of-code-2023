fn main() {
    let input = include_str!("./input.txt");
    let output_1 = part_1(input);
    println!("part 1: {}", output_1);
    let output_2 = part_2(input);
    println!("part 2: {}", output_2);
}

struct Schematic {
    grid: Vec<Vec<char>>,
}

impl Schematic {
    fn new(input: &str) -> Schematic {
        Schematic {
            grid: input
                .lines()
                .map(|l| l.trim().chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>(),
        }
    }

    fn get_pos(&self, x: usize, y: usize) -> char {
        self.grid[y][x]
    }

    fn check_column_for_symbol(&self, x: usize, y: usize) -> bool {
        if y > 0 && char_is_symbol(self.get_pos(x, y - 1)) {
            return true;
        }
        if y < self.grid.len() - 1 && char_is_symbol(self.get_pos(x, y + 1)) {
            return true;
        }
        if char_is_symbol(self.get_pos(x, y)) {
            return true;
        }
        return false;
    }

    fn check_point_for_symbol(&self, x: usize, y: usize) -> bool {
        if x > 0 && self.check_column_for_symbol(x - 1, y) {
            return true;
        }
        if x < self.grid[0].len() - 1 && self.check_column_for_symbol(x + 1, y) {
            return true;
        }
        self.check_column_for_symbol(x, y)
    }
}

fn char_is_symbol(c: char) -> bool {
    !c.is_digit(10) && c != '.'
}

enum ParseState {
    Ready,
    Parsing,
}

fn part_1(input: &str) -> String {
    let schematic = Schematic::new(input);
    // use a simple state machine to parse part numbers and skip wo symbol near
    // parse lines right to left to make it easier to parse numbers procedurally
    let mut part_number_sum = 0;
    let mut state = ParseState::Ready;
    let mut number = Vec::new();
    let mut found_symbol = false;
    for (y, line) in schematic.grid.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            match state {
                ParseState::Ready => {
                    if c.is_digit(10) {
                        number.clear();
                        found_symbol = false;
                        state = ParseState::Parsing;
                    }
                }
                _ => {}
            }
            match state {
                ParseState::Parsing => {
                    if let Some(digit) = c.to_digit(10) {
                        number.push(digit);
                        found_symbol = found_symbol || schematic.check_point_for_symbol(x, y);
                    }
                    if !c.is_digit(10) || x == schematic.grid[0].len() - 1 {
                        let mut part_number = 0;
                        for (place, digit) in number.iter().rev().enumerate() {
                            part_number += 10_u32.pow(place as u32) * digit;
                        }
                        if found_symbol {
                            part_number_sum += part_number;
                        }
                        state = ParseState::Ready;
                    }
                }
                _ => {}
            }
        }
    }
    part_number_sum.to_string()
}

fn part_2(input: &str) -> String {
    "?".to_string()
}

#[test]
fn test_schematic_matches_input() {
    let input = include_str!("./input.txt");
    let schematic = Schematic::new(input);
    assert_eq!(schematic.get_pos(0, 0), '.');
    assert_eq!(schematic.get_pos(0, 90), '7');
    assert_eq!(schematic.get_pos(5, 90), '@');
    assert_eq!(schematic.get_pos(139, 139), '.');
}

#[test]
fn test_example() {
    let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    assert_eq!(part_1(input), "4361");
}

#[test]
fn test_example_with_whitespace() {
    let input = "467..114..
...*......                           
                                       ..35..633. 
......#...
617*......   
.....+.58.
  ..592.....
......755.
 ...$.*....
.664.598..";
    assert_eq!(part_1(input), "4361");
}

#[test]
fn test_start_parsing() {
    let input = ".........*
.......100
..........
.....100..
10@.....*.
..........
.)...100*.
10........
..706*....";
    assert_eq!(part_1(input), "1026");
}

#[test]
fn test_overlapping_line() {
    let input = ".........*
100*...100
.......100
..........
......*100
100.......";
    assert_eq!(part_1(input), "300");
}

#[test]
fn test_first_lines() {
    let input = ".........................3.......................................94...............806....................596.........793...........186......
.../..........*574.587..*........161......904.......412.........*.................*.................................=.....637.%......*......
..614..831..33.....*...........@....*398..&.....690*............183.........503..916..790................................*.....256....632...
.........*.................332..374......................%184...........467..........*....969.632.......26....457......440.........@........
..........535...541=........*.......696..@..323..................93=.......*.......30......*....*......-........+.222$.......458.817....66..";
    assert_eq!(part_1(input), "16619");
}
