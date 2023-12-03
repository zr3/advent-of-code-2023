fn main() {
    let input = include_str!("./input.txt");
    let output_1 = part_1(input);
    println!("part 1: {}", output_1);
    let output_2 = part_2(input);
    println!("part 2: {}", output_2);
}

fn part_1(input: &str) -> String {
    input.to_string()
}

fn part_2(input: &str) -> String {
    "?"
}

#[test]
fn test() {
    assert!(true)
}
