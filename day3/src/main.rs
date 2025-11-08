use regex::Regex;
use std::fs;


fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input file");

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let sum: i32 = re.captures_iter(&input)
    .map(|cap| {
        let a = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let b = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
        a * b
    })
    .sum();

    println!("Sum of all valid multiplications: {}", sum);
}
