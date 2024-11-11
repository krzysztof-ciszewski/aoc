use fancy_regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn fourth() {
    let file = File::open("src/input/fourth.txt").expect("Could not open file");
    let puzzle = BufReader::new(&file).lines();
    let lines = puzzle.flatten().collect::<Vec<String>>();

    let mut freq = vec![1; lines.len()];
    let mut sum = 0;

    for (line_num, line) in lines.iter().enumerate() {
        let parts = line.split(':').nth(1).unwrap().split('|').collect::<Vec<&str>>();

        let re = Regex::new(r"(\d+)").unwrap();

        let winners = re.captures_iter(parts[0])
            .flatten()
            .collect::<Vec<_>>()
            .iter()
            .map(|e| e.get(1).unwrap().as_str().parse::<u32>().unwrap())
            .collect::<HashSet<u32>>();

        let actuals = re.captures_iter(parts[1])
            .flatten()
            .collect::<Vec<_>>()
            .iter()
            .map(|e| e.get(1).unwrap().as_str().parse::<u32>().unwrap())
            .collect::<HashSet<u32>>();

        let winning_num = winners.intersection(&actuals).count();

        if winning_num == 0 {
            continue;
        }

        let mut score = 1;
        for _ in 1..winning_num {
            score *= 2;
        }
        sum += score;

        for i in 0..winning_num {
            freq[line_num + i + 1] += freq[line_num];
        }
    }

    println!("Part 1: {}", sum);
    println!("Part 2: {}", freq.iter().sum::<usize>());
}