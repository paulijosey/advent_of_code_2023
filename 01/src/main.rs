use std::fs;
use std::collections::HashMap;

fn main() {
    let mut result = 0;
    // --snip--
    let file_path = "data/input.txt";
    println!("In file {}", file_path);

    // get data and save it to a vector
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines: Vec<String> = contents.lines().map(|line| line.to_string()).collect();

    // parse each line
    for line in &lines {
        let mut integer_string = String::new();
        integer_string.push(parse_string_for_integer(&line));

        // reverse string and do this again
        let reversed_line = line.chars().rev().collect::<String>();
        integer_string.push(parse_string_for_integer(&reversed_line));

        // println!("Integer: {}", integer_string);
        let integer: i32 = integer_string
            .trim()
            .parse()
            .expect("Unable to parse string to integer");
        result += integer;
    }
    println!("Result 01: {}", result);

    // Part 2
    result = 0;
    let digit_num: Vec<String> = vec![
        "1", "2", "3", "4", "5", "6", "7", "8", "9",
    ]
    .iter()
    .map(|&word| word.to_string())
    .collect();
    let digit_str: Vec<String> = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]
    .iter()
    .map(|&word| word.to_string())
    .collect();
    let reversed_digit_str: Vec<String> = digit_str
        .iter()
        .map(|word| word.to_string().chars().rev().collect::<String>())
        .collect();

    for line in &lines {
        let mut integer_string = String::new();
        integer_string.push(parse_string_for_integer_str(
            &line, 
            &digit_str, 
            &digit_num,
        ));

        // reverse string and do this again
        let reversed_line = line.chars().rev().collect::<String>();
        integer_string.push(parse_string_for_integer_str(
            &reversed_line,
            &reversed_digit_str,
            &digit_num,
        ));

        let integer: i32 = integer_string
            .trim()
            .parse()
            .expect("Unable to parse string to integer");
        result += integer;
    }
    println!("Result 02: {}", result);
}

fn parse_string_for_integer(line: &String) -> char {
    for ch in line.chars() {
        if ch.is_digit(10) {
            // finish when you found the first one
            return ch;
        }
    }
    return ' ';
}

fn parse_string_for_integer_str(line: &String, digits_str: &Vec<String>, digits_num: &Vec<String>) -> char {
    let mut res: Vec<String> = Vec::new();
    let mut res_idx: Vec<usize> = Vec::new();

    // create a mapping
    let number_mapping: HashMap<_, _> = digits_str.iter().zip(digits_num.iter()).collect();
    // first check for the words
    for word in digits_str.iter() {
        if let Some(index) = line.find(word) {
            res.push(number_mapping.get(word).unwrap().to_string());
            res_idx.push(index);
        }
    }

    // then check for the digits
    for word in digits_num.iter() {
        if let Some(index) = line.find(word) {
            res.push(word.to_string());
            res_idx.push(index);
        }
    }

    // now select the smallest index
    if let Some(&min_index) = res_idx.iter().min() {
        return res[res_idx.iter().position(|&r| r == min_index).unwrap()].chars().next().unwrap();
    }

    return ' ';
}
