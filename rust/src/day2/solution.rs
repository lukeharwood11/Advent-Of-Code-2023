use std::fs;
use std::collections::HashMap;
use regex::Regex;

pub fn run() {
    // load the test file in a vector
    let file = fs::read_to_string("./inputs/day2/input.txt").unwrap();
    let mut color_map: HashMap<&'static str, u8> = HashMap::new();
    color_map.insert("red", 12);
    color_map.insert("green", 13);
    color_map.insert("blue", 14);
    
    let sum = file.lines().fold(0, |acc, line| {
        let regex = Regex::new(r"Game (\d+): (.*)").unwrap();
        let capture = regex.captures(line).unwrap();
        let id = capture.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let values: &str = capture.get(2).unwrap().as_str();
        for value in values.split(|v| v == ',' || v == ';').collect::<Vec<_>>() {
            let mut buffer: String = String::new();
            let mut iter = value.trim().chars();
            loop {
                let char: char = iter.next().unwrap();
                if char.is_digit(10) {
                    buffer.push(char);
                } else if char.is_whitespace() {
                    break;
                }
            }
            let color = iter.collect::<String>();
            let val = buffer.parse::<u8>().unwrap();
            let max: u8 = *color_map.get(color.as_str()).unwrap();
            if val > max {
                return acc;
            }
        }
        acc + id
    });
    println!("{sum}");
    // 2727
}


pub fn run2() {
    // load the test file in a vector
    let file = fs::read_to_string("./inputs/day2/input.txt").unwrap();
    
    let sum = file.lines().fold(0, |acc, line| {
        let regex = Regex::new(r"Game (\d+): (.*)").unwrap();
        let capture = regex.captures(line).unwrap();
        let values: &str = capture.get(2).unwrap().as_str();
        let mut max_values: HashMap<&str, i32> = HashMap::new();
        max_values.insert("red", 0);
        max_values.insert("blue", 0);
        max_values.insert("green", 0);
        for value in values.split(|v| v == ',' || v == ';').collect::<Vec<_>>() {
            let mut buffer: String = String::new();
            let mut iter = value.trim().chars();
            loop {
                let char: char = iter.next().unwrap();
                if char.is_digit(10) {
                    buffer.push(char);
                } else if char.is_whitespace() {
                    break;
                }
            }
            let color = iter.collect::<String>();
            let val = buffer.parse::<i32>().unwrap();
            let max_value = max_values.get_mut(color.as_str()).unwrap();
            if val > *max_value {
                *max_value = val;
            }
        }
        acc + max_values.values().product::<i32>()
    });
    println!("{sum}");
    // 56580
}