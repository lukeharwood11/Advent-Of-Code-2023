use regex::Regex;
use std::fs;
use std::time::Instant;

pub fn run() {
    let start = Instant::now();
    let number_strings: Vec<&str> = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let file = fs::read_to_string("./inputs/day1/input.txt").unwrap();
    let sum = file.lines().fold(0, |acc, line| {
        // first get the indices of each match and place it in a vector
        let mut matches: Vec<(usize, String)> = vec![];
        for number in &number_strings {
            let reg = Regex::new(number).unwrap();
            for value in reg.find_iter(line) {
                let number_value = number_strings.iter().position(|n| n == number).unwrap() + 1;
                matches.push((value.start(), number_value.to_string()));
            }
        }
        for (i, c) in line.bytes().enumerate() {
            if c.is_ascii_digit() {
                matches.push((i, c.escape_ascii().to_string()));
            }
        }
        // sort to ensure they remain in order
        matches.sort_by_key(|&(i, _)| i);
        let values = matches
            .iter()
            .map(|(_, value)| value.to_owned())
            .collect::<Vec<_>>();
        // grab the first and last and parse it into an int
        let mut x = values.get(0).unwrap().to_owned();
        let y = values.last().unwrap().to_owned();
        x.push_str(y.as_str());
        acc + x.parse::<i32>().unwrap()
    });
    let end = Instant::now();
    println!("Sum: {}, Time: {:?}", sum, end - start);
}