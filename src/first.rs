use std::fs::File;
use std::io::{BufRead, BufReader};

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn first() {
    let file = File::open("src/input/first.txt").expect("Could not open file");
    let puzzle = BufReader::new(file).lines();

    let mut sum = 0;

    for line in puzzle.flatten() {
        sum += find_text_num(line.chars().collect());
    }

    println!("Sum: {}", sum);
}

fn find_text_num(line: Vec<char>) -> i32 {
    let mut number = Vec::new();
    let mut left: usize = 0;
    let mut right: usize = line.len() - 1;

    while left <= line.len() - 1 {
        if number.len() > 0 {
            break;
        }

        if line[left].is_numeric() {
            number.push(line[left]);
            break;
        }

        let digit_words = digit_starts_with(line[left]);
        if digit_words.len() == 0 {
            left += 1;
            continue;
        }

        for digit_word in digit_words {
            let dw: Vec<char> = digit_word.chars().collect();

            let mut offset = 0;
            let mut buffer = String::new();
            let mut tmp_left = left;
            while offset <= dw.len() - 1 && left <= line.len() - 1 && line[tmp_left] == dw[offset] {
                buffer.push(line[tmp_left]);
                offset += 1;
                tmp_left += 1;
            }

            if is_digit(&buffer) {
                number.push(digit_word_to_digit(&buffer));
                break;
            }
        }

        left += 1;
    }

    while right >= left {
        if number.len() > 1 {
            break;
        }

        if line[right].is_numeric() {
            number.push(line[right]);
            break;
        }

        let digit_words = digit_ends_with(line[right]);
        if digit_words.len() == 0 {
            right -= 1;
            continue;
        }

        for digit_word in digit_words {
            let dw: Vec<char> = digit_word.chars().collect();

            let mut offset = dw.len() - 1;
            let mut tmp_right = right;

            let mut buffer = String::new();

            while offset >= 0 && right > left && line[tmp_right] == dw[offset] {
                buffer.push(line[tmp_right]);
                if offset == 0 {
                    break;
                }
                offset -= 1;
                tmp_right -= 1;
            }

            buffer = buffer.chars().rev().collect();

            if is_digit(&buffer) {
                number.push(digit_word_to_digit(&buffer));
                break;
            }
        }

        right -= 1;
    }

    number.into_iter().collect::<String>().parse().unwrap()
}

fn digit_word_to_digit(digit: &String) -> char {
    for (i, element) in DIGITS.iter_mut().enumerate() {
        if element == digit {
            return (i + 1).to_string().chars().next().unwrap();
        }
    }

    '0'
}

fn is_digit(possible_digit: &String) -> bool {
    for digit in DIGITS.iter() {
        if digit == possible_digit {
            return true;
        }
    }

    false
}

fn digit_starts_with(char: char) -> Vec<String> {
    let mut starts_with: Vec<String> = Vec::new();

    for digit in DIGITS.iter() {
        if digit.starts_with(char) {
            starts_with.push(digit.to_string());
        }
    }

    starts_with
}

fn digit_ends_with(char: char) -> Vec<String> {
    let mut ends_with: Vec<String> = Vec::new();
    for digit in DIGITS.iter() {
        if digit.ends_with(char) {
            ends_with.push(digit.to_string());
        }
    }

    ends_with
}

fn find_num(line: Vec<char>) -> i32 {
    let mut number = Vec::new();
    let mut left: usize = 0;
    let mut right: usize = line.len() - 1;

    while !line[left].is_numeric() {
        left += 1;
    }

    number.push(line[left]);

    while !line[right].is_numeric() {
        right -= 1;
    }

    number.push(line[right]);

    number.into_iter().collect::<String>().parse().unwrap()
}
