use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

pub fn day3_a() -> Result<String, Box<dyn Error>> {
    let file = File::open("/tmp/3.txt")?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|line| {line.unwrap()}).collect();


    let mut valid_nums: Vec<u32> = Vec::new();

    for i in 0..lines.len() {
        let line = lines.get(i).expect("Failed to get line").clone();
        if line.is_empty() {continue}
        let nums = extract_numbers_with_positions(line.clone());
        for (num, start, end) in nums {
            // Check symbols before the number
            if start > 0 && is_char_symbol(line.chars().nth((start - 1) as usize).unwrap()) {
                valid_nums.push(num);
                continue
            }
            // Check symbols after the number
            if end < (line.len() as u32 - 1) && is_char_symbol(line.chars().nth((end + 1) as usize).unwrap()) {
                valid_nums.push(num);
                continue
            }
            // Get the string up if not the first line
            if i > 0 {
                let up = lines.get(i-1).expect("Failed to get previous line").clone();
                let t_start = if start > 0 {start -1  } else { 0 };
                let t_end = if end < (line.len() as u32 - 1) { end + 1  } else { end };
                let extracted = extract_lines_in_pos(up, t_start, t_end);
                if contains_symbol(extracted) {
                    valid_nums.push(num);
                    continue
                }
            }
            // Get the line down if ot the last line
            if i < lines.len() - 1 {
                let down = lines.get(i+1).expect("Failed to get next line").clone();
                let t_start = if start > 0 {start -1  } else { 0 };
                let t_end = if end < (line.len() as u32 - 1) { end + 1  } else { end };
                let extracted = extract_lines_in_pos(down, t_start, t_end);
                if contains_symbol(extracted) {
                    valid_nums.push(num);
                    continue
                }
            }

        }
    }

    let sum: u32 = valid_nums.iter().sum();

    Ok(sum.to_string())
}

fn extract_numbers_with_positions(string: String) -> Vec<(u32, u32, u32)> { // Vec<(extracted_num, first_pos, last_pos)>
    let s: &str = &*string;
    let re = Regex::new(r"\d+").expect("Failed to create regex");
    let matches: Vec<_> = re.find_iter(s).collect();

    let numbers: Vec<(u32, u32, u32)> = matches
        .iter()
        .filter_map(|m| {
            m.as_str()
                .parse()
                .map(|num| (num, m.start() as u32, (m.end() - 1) as u32))
                .ok()
        })
        .collect();

    numbers
}

pub fn extract_lines_in_pos(string : String, start:u32, end:u32) -> String {
    let s = &*string;
    let extracted: String = s.chars().skip(start as usize).take((end - start) as usize + 1).collect();
    extracted
}

fn is_char_symbol(c: char) -> bool {
    (!c.is_digit(10) && c != '.')
}

fn contains_symbol(s: String) -> bool {
    s.chars().any(|c| !c.is_digit(10) && c != '.')
}

pub fn day3_b() -> Result<String, Box<dyn Error>> {
    let file = File::open("/tmp/3.txt")?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|line| {line.unwrap()}).collect();

    let array_x = lines.get(0).unwrap().len() as u32;
    let array_y = lines.len() as u32;

    let mut gears_ratios: Vec<u32> = Vec::new();

    for i in 0..lines.len() {
        let line = lines.get(i).expect("Failed to get line").clone();
        if line.is_empty() {continue}
        let gears = find_gears_positions(line.clone());
        for gear in gears {
            let mut ratios: Vec<u32> = Vec::new();
            // Check symbols before the pos
            if gear > 0 && line.chars().nth((gear - 1) as usize).unwrap().is_digit(10) {
                let (num_str, _) = line.split_at(gear as usize);
                let num = extract_numbers_with_positions(num_str.into()).last().expect("Failed to extract number").0;
                ratios.push(num);
            }
            // Check symbols after the number
            if gear < (line.len() - 1) as u32 && line.chars().nth((gear + 1) as usize).unwrap().is_digit(10) {
                let (_, num_str) = line.split_at(gear as usize + 1);
                let num = extract_numbers_with_positions(num_str.into()).first().expect("Failed to extract number").0;
                ratios.push(num);
            }
            // Get the string up if not the first line
            if i > 0 {
                let up = lines.get(i-1).expect("Failed to get previous line").clone();
                let nums = extract_numbers_with_positions(up.clone());
                for (num, start, end) in nums {
                    let t_start = if start > 0 {start -1  } else { 0 };
                    let t_end = if end < (line.len() as u32 - 1) { end + 1  } else { end };
                    if t_start <= gear && gear <= t_end {
                        ratios.push(num)
                    }
                }
            }
            // Get the line down if ot the last line
            if i < lines.len() - 1 {
                let down = lines.get(i+1).expect("Failed to get previous line").clone();
                let nums = extract_numbers_with_positions(down.clone());
                for (num, start, end) in nums {
                    let t_start = if start > 0 {start -1  } else { 0 };
                    let t_end = if end < (line.len() as u32 - 1) { end + 1  } else { end };
                    if t_start <= gear && gear <= t_end {
                        ratios.push(num)
                    }
                }
            }

            if ratios.len() == 2 {
                gears_ratios.push(ratios.get(0).unwrap() * ratios.get(1).unwrap())
            }
        }
    }

    let sum: u32 = gears_ratios.iter().sum();

    Ok(sum.to_string())
}

fn find_gears_positions(string: String) -> Vec<u32> {
    let char_vector: Vec<char> = string.chars().collect();
    let mut result: Vec<u32> = Vec::new();
    for i in 0..char_vector.len() {
        if char_vector.get(i).unwrap().clone() == '*' {
            result.push(i as u32);
        }
    }
    result
}