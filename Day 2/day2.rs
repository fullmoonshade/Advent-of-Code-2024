use std::fs::File;
use std::io::{self, BufRead};


fn is_safe(levels: &Vec<i32>) -> bool {
    if levels.len() < 2 {
        return true;
    }

    let mut increasing = true;
    let mut decreasing = true;

    for i in 1..levels.len() {
        let diff = levels[i] - levels[i - 1];
        let abs_diff = diff.abs();

        if abs_diff < 1 || abs_diff > 3 {
            return false;
        }

        if diff > 0 {
            decreasing = false;
        }
        if diff < 0 {
            increasing = false;
        }
        
    }
    return increasing || decreasing;
}

fn main() -> io::Result<()> {
        let path = "input.txt";
    
        let file = File::open(path)?;
        let reader = io::BufReader::new(file);
    
        let mut safe_count = 0;
    
        for line in reader.lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }
    
            // Parse numbers into a Vec<i32>
            let levels: Vec<i32> = line
                .split_whitespace()
                .filter_map(|s| s.parse::<i32>().ok())
                .collect();
    
            if is_safe(&levels) {
                safe_count += 1;
            }
        }
    
        println!("Safe reports: {}", safe_count);
    
        Ok(())
    }
