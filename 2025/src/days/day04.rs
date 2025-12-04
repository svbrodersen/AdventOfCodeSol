use std::fs;

fn count_ats(inp: &[char]) -> usize {
    inp.iter()
        .fold(0, |count, c| if *c == '@' { count + 1 } else { count })
}

fn part1(lines: &[String]) {
    let res = lines.iter().fold(0, |count, line| {
        handle_line(count, line)
    });
    println!("Result: {res}");
}

fn handle_line(mut count: i32, line: &str) -> i32 {
    let chars: Vec<char> = line.chars().collect();
    for (i, c) in chars.iter().enumerate() {
        if *c == '@' {
            let num_neighbours = count_ats(&chars[0..i]) + count_ats(&chars[i + 1..]);
            if num_neighbours < 4 {
                count += 1;
            }
        }
    }
    count
}

/// # Panics
pub fn solve() {
    let lines: Vec<String> = fs::read_to_string("inputs/day04.txt")
        .unwrap()
        .lines()
        .map(std::string::ToString::to_string)
        .collect();
    part1(&lines);
    // part2(&lines);
}
