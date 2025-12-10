use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::env;
//use crate::part1;

pub mod part1;
pub mod part2;


fn main_part1(file_path: &str) -> std::io::Result<()>{
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut joltage_sum: i32 = 0;

    for line in reader.lines() {
        let line_str = line?;
        let line_joltage = part1::get_max_line_joltage(&line_str).expect("Input string should only ever use base 10 numbers");
        joltage_sum += line_joltage;
    } 
    println!("Total Joltage: {}", joltage_sum);
    Ok(())
}
fn main_part2(file_path: &str) -> std::io::Result<()>{
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut joltage_sum: i64 = 0;

    for line in reader.lines() {
        let line_str = line?;
        let line_joltage = part2::get_max_line_joltage(&line_str, 12).expect("Input string should only ever use base 10 numbers");
        joltage_sum += line_joltage;
    } 
    println!("Total Joltage: {}", joltage_sum);
    Ok(())
}


fn main() -> std::io::Result<()>{
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            let test_val: &str = "234234234234278";
            let joltage = part2::get_max_line_joltage(test_val, 12).unwrap(); 
            println!("Test Joltage: {}", joltage);
        }
        2 => {
            let _ = main_part2(&args[1])?;
        },
        _ => {
            println!("function takes one argument: input filepath. Recieved {}", args.len())
        }
    }

    Ok(())
}
