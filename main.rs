use crate::io::BufReader;
use std::fs::File;
use std::io::{self, BufRead};

fn open_file(path_to_file: &str) -> io::Result<BufReader<File>> {
    let file = File::open(path_to_file)?;
    let reader = BufReader::new(file);
    Ok(reader)
}

fn same_elements_number(reader: BufReader<File>) -> Vec<i32> {
    let mut repetitions: Vec<i32> = Vec::new();
    for line_result in reader.lines() {
        match line_result {
            Ok(line) => {
                let parts: Vec<&str> = line.trim().split('|').collect();

                let parts_zero: Vec<&str> = parts[0].split(":").collect();

                if parts.len() > 1 {
                    let first_row: Vec<i32> = parts_zero[1]
                        .split_whitespace()
                        .map(|s| s.parse::<i32>().unwrap())
                        .collect();

                    let second_row: Vec<i32> = parts[1]
                        .split_whitespace()
                        .map(|s| s.parse::<i32>().unwrap())
                        .collect();

                    let common_elements_count =
                        second_row.iter().filter(|&x| first_row.contains(x)).count();

                    repetitions.push(common_elements_count as i32);
                }
            }

            Err(err) => {
                eprintln!("Error reading line: {}", err);
            }
        }
    }

    repetitions
}

struct Card {
    id: u32,
    matching_numbers: u32,
}

impl Card {
    fn new(id: u32, matching_numbers: u32) -> Self {
        Card {
            id,
            matching_numbers,
        }
    }
}

fn main() {
    let path_to_file = "/home/peteshko/advent_of_code/winning_numbers.txt";
    let mut repeatitions: Vec<i32> = Vec::new();
    match open_file(path_to_file) {
        Ok(reader) => {
            repeatitions = same_elements_number(reader);
        }
        Err(err) => {
            eprintln!("Error opening file: {}", err);
        }
    }

    let mut ones_array: Vec<i32> = vec![1; repeatitions.len()];

    for (idx, i) in repeatitions.iter().enumerate() {
        let limit = idx + *i as usize;

        let limit = limit.min(ones_array.len() - 1);
        let step = ones_array[idx];

        for j in (idx + 1)..=limit {
            ones_array[j] += step;
        }
    }

    let sum: i32 = ones_array.iter().sum();
    println!("{:?}", sum);
}
