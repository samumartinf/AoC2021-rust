use std::collections::VecDeque;
use std::fs;

pub fn day1() {
    let file_path = "inputs/01.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    process_text_day01(&contents);
    process_text_day01_sliding(3, &contents);
}

fn process_text_day01(contents: &str) {
    let mut count = -1;
    let mut prev = 0;
    for line in contents.lines() {
        let current_depth = line.parse::<i32>().unwrap();
        if current_depth > prev {
            count += 1;
        }
        prev = current_depth;
    }
    println!("Day01: There are {} increases", count);
}

fn process_text_day01_sliding(window_len: i32, contents: &str) {
    let mut count = -window_len;
    let mut prev = 0;
    let init_array: [i32; 3] = [0; 3];
    let mut window = VecDeque::from(init_array);

    for line in contents.lines() {
        // Option 1 -> write all numbers into array and procees array
        // This will be memory intensive
        //
        // Option 2 -> Create a buffer and reduce the first three counts as the
        // measurements are all positive integers. (i.e. count = -3)
        //
        let current_depth = line.parse::<i32>().unwrap();
        window.pop_front();
        window.push_back(current_depth);

        let sum: i32 = window.iter().sum();

        if sum > prev {
            count += 1;
        }

        prev = sum;
    }
    println!("Day01: There are {} weighted average increases", count);
}

// Day 02 -------------
pub fn day2() {
    let file_path = "inputs/02.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    process_text_day02(&contents);
    process_text_day02_pt2(&contents);
}

fn process_text_day02(contents: &str) {
    let mut depth: i32 = 0;
    let mut distance: i32 = 0;

    for line in contents.lines() {
        let mut split = line.split_whitespace();
        let direction = split.next().unwrap();
        let magnitude = split.next().unwrap().parse::<i32>().unwrap();

        match direction {
            "forward" => distance += magnitude,
            "up" => depth -= magnitude,
            "down" => depth += magnitude,
            _ => (),
        }
    }
    println!(
        "Day02: The final depth is {} and distance {}. The resutl is {}",
        depth,
        distance,
        depth * distance
    );
}

fn process_text_day02_pt2(contents: &str) {
    let mut aim: i32 = 0;
    let mut depth: i32 = 0;
    let mut distance: i32 = 0;

    for line in contents.lines() {
        let mut split = line.split_whitespace();
        let direction = split.next().unwrap();
        let magnitude = split.next().unwrap().parse::<i32>().unwrap();

        match direction {
            "forward" => {
                distance += magnitude;
                depth += aim * magnitude;
            }
            "up" => aim -= magnitude,
            "down" => aim += magnitude,
            _ => (),
        }
    }
    println!(
        "Day02: The final depth is {} and distance {}. The result is {}",
        depth,
        distance,
        depth * distance
    );
}

//----- Day 3 -------

pub fn day3() {
    let file_path = "inputs/03.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    process_text_day03(&contents);
}

fn process_text_day03(contents: &str) {
    let mut sum_vec: Vec<f64> = Vec::new();
    for line in contents.lines() {
        for _ in 0..String::from(line).len() {
            sum_vec.push(0f64);
        }
        break;
    }

    let mut line_count: f64 = 0f64;

    for line in contents.lines() {
        line_count += 1f64;

        for i in 0..line.len() {
            sum_vec[i] += line.chars().nth(i).unwrap().to_digit(10).unwrap() as f64;
        }
    }

    let sum_vec_iter = sum_vec.iter();
    let result_gamma: Vec<f64> = sum_vec_iter.map(|x| x / line_count).collect();
    let binary_gamma: Vec<i32> = result_gamma.iter().map(|x| x.round() as i32).collect();
    let binary_espsilon: Vec<i32> = result_gamma
        .iter()
        .map(|x| ((x - 1f64).round() * -1f64) as i32)
        .collect();
    let gamma_string = join_vec_to_string(&binary_gamma);
    let epsilon_string: String = join_vec_to_string(&binary_espsilon);
    let epsilon_dec: i32 = binary_to_number(&epsilon_string);
    let gamma_dec: i32 = binary_to_number(&gamma_string);
    println!(
        "Day03: Gamma output is {:?}. The Epsilon output is {}. The final result is {}.",
        gamma_dec,
        epsilon_dec,
        epsilon_dec * gamma_dec
    );
}

fn join_vec_to_string(vector: &Vec<i32>) -> String {
    let mut string_vector = vec!["".to_string()];
    for item in vector {
        string_vector.push(item.to_string());
    }
    let final_string = string_vector.iter().cloned().collect::<String>();
    return final_string;
}

fn binary_to_number(binstr: &str) -> i32 {
    let intval = isize::from_str_radix(binstr, 2).unwrap() as i32;
    return intval;
}
