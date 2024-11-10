use std::cmp::{max, min};
use std::fs::File;
use std::io::{BufRead, BufReader, Seek};

#[derive(Debug)]
#[derive(Clone, PartialEq)]
struct Number {
    number: String,
    x: (u32, u32),
    y: u32,
}
struct Gear {
    x: u32,
    y: u32,
}

pub fn third() {
    let mut file = File::open("src/input/third.txt").expect("Could not open file");
    let puzzle = BufReader::new(&file).lines();

    let mut numbers: Vec<Number> = Vec::new();
    let mut gears: Vec<Gear> = Vec::new();

    for (line_idx, line) in puzzle.enumerate() {
        let line = line.unwrap();
        let mut chars = line.chars().enumerate().peekable();
        while let Some((x, char)) = chars.next() {
            if char.is_numeric() {
                let mut buffer = String::new();
                buffer.push(char);
                let mut end_index = x;
                while let Some((nIdx, next_char)) = chars.peek() {
                    if next_char.is_numeric() {
                        buffer.push(next_char.clone());
                        end_index = nIdx.clone();
                        chars.next();
                    } else {
                        break;
                    }
                }

                numbers.push(Number {
                    number: buffer,
                    x: (x as u32, end_index as u32),
                    y: line_idx as u32,
                });
            }

            if char == '*' {
                gears.push(Gear {
                    x: x as u32,
                    y: line_idx as u32,
                })
            }
        }
    }

    part2(&mut file, &mut numbers, &mut gears);
}

fn part2(file: &mut File, numbers: &mut Vec<Number>, gears: &mut Vec<Gear>) {
    file.rewind().expect("Cannot rewind file");
    let puzzle = BufReader::new(&*file).lines();
    let lines = puzzle.flatten().collect::<Vec<String>>();
    let mut sum: u32 = 0;
    for gear in gears.iter() {
        let mut adjacent_numbers: Vec<Number> = Vec::new();
        let line = lines.get(gear.y as usize).unwrap();
        let start = max(gear.x as i32 - 1, 0);
        let end = min(gear.x + 1, (line.len() - 1) as u32);
        if gear.y > 0 {
            for n in start as u32..=end {
                for number in numbers.iter() {
                    if (number.x.0..=number.x.1).contains(&n) && number.y == gear.y - 1 {
                        if !adjacent_numbers.contains(number) {
                            adjacent_numbers.push(number.clone());
                        }
                    }
                }
            }
        }
        for n in start as u32..=end {
            for number in numbers.iter() {
                if (number.x.0..=number.x.1).contains(&n) && number.y == gear.y {
                    if !adjacent_numbers.contains(number) {
                        adjacent_numbers.push(number.clone());
                    }
                }
            }
        }

        if gear.y + 1 <= lines.len() as u32 - 1 {
            for n in start as u32..=end {
                for number in numbers.iter() {
                    if (number.x.0..=number.x.1).contains(&n) && number.y == gear.y + 1 {
                        if !adjacent_numbers.contains(number) {
                            adjacent_numbers.push(number.clone());
                        }
                    }
                }
            }
        }

        if adjacent_numbers.len() == 2 {
            sum += adjacent_numbers[0].number.parse::<u32>().unwrap() * adjacent_numbers[1].number.parse::<u32>().unwrap();
        }
    }

    println!("{}", sum);
}

fn part1(file: &mut File, numbers: &mut Vec<Number>) {
    file.rewind().expect("Cannot rewind file");
    let puzzle = BufReader::new(file).lines();
    let lines = puzzle.flatten().collect::<Vec<String>>();
    let mut sum = 0;
    for number in numbers.iter() {
        if number.y > 0 {
            let line = lines.get((number.y - 1) as usize).unwrap();

            if has_symbol(&line, &number) {
                let num = number.number.parse::<u32>().unwrap();
                sum += num;
                continue;
            }
        }

        let line = lines.get(number.y as usize).unwrap();
        if has_symbol(&line, &number) {
            let num = number.number.parse::<u32>().unwrap();
            sum += num;
            continue;
        }

        if number.y + 1 <= lines.len() as u32 - 1 {
            let line = lines.get((number.y + 1) as usize).unwrap();
            if has_symbol(&line, &number) {
                let num = number.number.parse::<u32>().unwrap();
                sum += num;
                continue;
            }
        }
    }

    println!("{}", sum);
}

fn has_symbol(line: &str, number: &Number) -> bool {
    let start = max(number.x.0 as i32 - 1, 0);
    let end = min(number.x.1 + 1, (line.len() - 1) as u32);
    let char_vec = line.chars().collect::<Vec<char>>();
    for n in start as u32..=end {
        let c = char_vec.get(n as usize).unwrap();
        if is_symbol(*c) {
            return true;
        }
    }

    false
}

fn is_symbol(c: char) -> bool {
    c != '.' && !c.is_ascii_digit() && !c.is_ascii_whitespace()
}
