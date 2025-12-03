use std::{
    fs
};

fn part1(lines: &[String]) {
    let (_, res) = lines.iter().fold((50, 0), |(pos, count), line| {
        let chars = line;
        let (char, tmp_val) = chars.split_at(1);
        let val = tmp_val.parse::<i64>().unwrap();

        let new_pos = match char {
            "L" => (pos - val).rem_euclid(100),
            "R" => (val + pos).rem_euclid(100),
            _ => panic!("Unexpected character"),
        };
        if new_pos == 0 {
            (new_pos, count + 1)
        } else {
            (new_pos, count)
        }
    });
    println!("Result: {res}");
}

fn part2(lines: &[String]) {
    let (_, res) = lines.iter().fold((50, 0), |(mut pos, mut count), line| {
        let chars = line;
        let (letter, tmp_val) = chars.split_at(1);
        let mut val = tmp_val.parse::<i64>().unwrap();

        val = match letter {
            "L" => -val,
            "R" => val,
            _ => panic!("Unexpected character"),
        };

        if pos == 0 && val < 0 {
            count -= 1;
        }

        pos += val;
        count += pos.div_euclid(100).abs();
        pos = pos.rem_euclid(100);

        if pos == 0 && val < 0 {
            count += 1;
        }

        (pos, count)
    });
    println!("Result: {res}");
}

/// # Panics
pub fn solve() {
    let lines: Vec<String> = fs::read_to_string("inputs/day01.txt")
        .unwrap()
        .lines()
        .map(std::string::ToString::to_string)
        .collect();
    part1(&lines);
    part2(&lines);
}
