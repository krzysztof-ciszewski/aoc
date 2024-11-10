use fancy_regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

pub fn second() {
    let file = File::open("src/input/second.txt").expect("Could not open file");
    let puzzle = BufReader::new(file).lines();

    ksaruu_solution(puzzle);
}

fn ksaruu_solution(puzzle: Lines<BufReader<File>>) {
    let mut sum: u32 = 0;

    for line in puzzle.flatten() {
        let re = Regex::new(r"(?:Game\s)(?P<gameNr>\d+):\s|(?P<colorNr>\d+)\s(?P<colorName>blue|red|green)(?:[; ,]*)+").unwrap();

        let mut captures = re.captures_iter(line.as_str());
        let game_capture = captures.next().unwrap().unwrap();

        let game_nr = game_capture
            .name("gameNr")
            .unwrap()
            .as_str()
            .parse::<u8>()
            .unwrap();

        let mut possible = true;
        for capture in captures {
            let capture = capture.unwrap();
            let color_name = capture.name("colorName").unwrap().as_str();
            let color_nr = capture
                .name("colorNr")
                .unwrap()
                .as_str()
                .parse::<u8>()
                .unwrap();

            if color_name == "red" && color_nr > 12 {
                possible = false;
            }

            if color_name == "green" && color_nr > 13 {
                possible = false;
            }

            if color_name == "blue" && color_nr > 14 {
                possible = false;
            }
        }

        if possible {
            sum += game_nr as u32;
        }
    }

    println!("Sum: {}", sum);
}

fn ksaruu_solution_part2(puzzle: Lines<BufReader<File>>) {
    let mut sum: u32 = 0;

    for line in puzzle.flatten() {
        let re = Regex::new(r"(?:Game\s)(?P<gameNr>\d+):\s|(?P<colorNr>\d+)\s(?P<colorName>blue|red|green)(?:[; ,]*)+").unwrap();

        let captures = re.captures_iter(line.as_str());

        let mut max_red = 0;
        let mut max_blue = 0;
        let mut max_green = 0;

        for capture in captures {
            let capture = capture.unwrap();
            let color_name = capture.name("colorName").unwrap().as_str();
            let color_nr = capture
                .name("colorNr")
                .unwrap()
                .as_str()
                .parse::<u32>()
                .unwrap();

            match color_name {
                "red" => {
                    if color_nr > max_red {
                        max_red = color_nr;
                    }
                }
                "green" => {
                    if color_nr > max_green {
                        max_green = color_nr;
                    }
                }
                "blue" => {
                    if color_nr > max_blue {
                        max_blue = color_nr;
                    }
                }
                _ => {}
            }
        }

        sum += max_red * max_blue * max_green;
    }

    println!("Sum: {}", sum);
}

fn my_solution(puzzle: Lines<BufReader<File>>) {
    let mut sum: u32 = 0;

    for line in puzzle.flatten() {
        let re = Regex::new(r"Game (\d+):").unwrap();

        let line_str = line.as_str();
        let id = re
            .captures(line_str)
            .unwrap()
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .to_string()
            .parse::<u8>()
            .unwrap();

        let max_red = get_max_red(line_str);
        if max_red > 12 {
            continue;
        }
        let max_green = get_max_green(line_str);
        if max_green > 13 {
            continue;
        }
        let max_blue = get_max_blue(line_str);
        if max_blue > 14 {
            continue;
        }

        sum += id as u32;
    }

    println!("Sum: {}", sum);
}

fn get_max_red(line: &str) -> u8 {
    let re = Regex::new(r"(\d+) red").unwrap();

    get_max(line, re)
}

fn get_max_green(line: &str) -> u8 {
    let re = Regex::new(r"(\d+) green").unwrap();

    get_max(line, re)
}

fn get_max_blue(line: &str) -> u8 {
    let re = Regex::new(r"(\d+) blue").unwrap();

    get_max(line, re)
}

fn get_max(line: &str, re: Regex) -> u8 {
    let captures = re.captures_iter(line);

    let mut max = 0;
    for capture in captures {
        let val = capture
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse::<u8>()
            .unwrap();
        if val > max {
            max = val;
        }
    }

    max
}
