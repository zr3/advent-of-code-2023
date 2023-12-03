fn main() {
    let input = include_str!("./input.txt");
    let output_1 = part_1(input);
    println!("part 1: {}", output_1);
    let output_2 = part_2(input);
    println!("part 2: {}", output_2);
}

#[derive(PartialEq)]
enum CubeColor {
    Red,
    Blue,
    Green,
    Error,
}

struct Grab {
    num: u32,
    color: CubeColor,
}

impl Grab {
    fn new(num: u32, color: CubeColor) -> Grab {
        Grab { num, color }
    }
}

struct GameRecord {
    id: u32,
    rounds: Vec<Vec<Grab>>,
}

impl GameRecord {
    fn new(record: &str) -> GameRecord {
        let meta: Vec<&str> = record.split(':').collect();
        let id = meta[0]
            // [Game ##]
            .split(' ')
            .nth(1)
            .expect("input should guarantee")
            .parse::<u32>()
            .expect("input should guarantee");
        let round_records: Vec<&str> = meta[1].split(';').collect();
        let mut rounds = Vec::new();
        for round in round_records {
            let segments: Vec<&str> = round.split(',').collect();
            let grabs = segments
                .iter()
                .map(|s| {
                    Grab::new(
                        s.trim()
                            .split(' ')
                            .nth(0)
                            .expect("input should guarantee")
                            .parse::<u32>()
                            .expect("input should guarantee"),
                        match s.trim().split(' ').nth(1).expect("input should guarantee") {
                            "red" => CubeColor::Red,
                            "blue" => CubeColor::Blue,
                            "green" => CubeColor::Green,
                            _ => CubeColor::Error,
                        },
                    )
                })
                .collect();
            rounds.push(grabs);
        }
        GameRecord { id, rounds }
    }

    fn max(&self, color: CubeColor) -> u32 {
        let mut max = 0;
        for round in &self.rounds {
            let count = round
                .iter()
                .filter(|g| g.color == color)
                .map(|g| g.num)
                .sum();
            if count > max {
                max = count;
            }
        }
        max
    }

    fn get_power(&self) -> u32 {
        self.max(CubeColor::Red) * self.max(CubeColor::Green) * self.max(CubeColor::Blue)
    }
}

fn part_1(input: &str) -> String {
    let mut sum = 0;
    let lines = input.lines();
    for line in lines {
        let rec = GameRecord::new(line);
        if rec.max(CubeColor::Red) <= 12
            && rec.max(CubeColor::Green) <= 13
            && rec.max(CubeColor::Blue) <= 14
        {
            sum += rec.id;
        }
    }
    sum.to_string()
}

fn part_2(input: &str) -> String {
    let mut sum = 0;
    let lines = input.lines();
    for line in lines {
        let rec = GameRecord::new(line);
        sum += rec.get_power();
    }
    sum.to_string()
}
