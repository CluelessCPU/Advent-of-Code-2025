use std::fs::File;
use std::io::{self, prelude::*, BufReader};

const DIAL_POSITIONS: i32 = 100;
const DIAL_START: i32 = 50;
const TARGET_POSITION: i32 = 0;

fn main_part_1() -> io::Result<()> {
    let file = File::open("./day1.1.txt")?;
    let reader = BufReader::new(file);
    let mut position: i32 = DIAL_START;
    let mut target_hits: i32 = 0;

    for line in reader.lines() {
        let line_str = line?;
        let val_string = &line_str[1..];
        let val = val_string.parse::<i32>().unwrap();


        if line_str.starts_with("R") {
            position += val
        } else {
            position -= val
        }
        position = position.rem_euclid(DIAL_POSITIONS);
        
        if position == TARGET_POSITION {
            target_hits += 1
        }
        //print!("{}", position)
    }

    print!("{}", target_hits);

    Ok(())
}

fn main_part_2() -> io::Result<()> {
    let file = File::open("./day1.1.txt")?;
    let reader = BufReader::new(file);
    let mut position: i32 = DIAL_START;
    let mut target_hits: i32 = 0;

    for line in reader.lines() {
        let line_str = line?;
        let val_string = &line_str[1..];
        let mut val = val_string.parse::<i32>().unwrap();


        if line_str.starts_with("L") {
            val = -val
        }

        let new_position = position + val;

        if (new_position <= 0) && (position != 0) {
            target_hits += 1
        }

        let full_rotations = (new_position / DIAL_POSITIONS).abs();
        target_hits += full_rotations;

        position = new_position.rem_euclid(DIAL_POSITIONS);


        //print!("{}", position)
    }

    print!("{}", target_hits);

    Ok(())
}

fn main() -> io::Result<()>{
    let _ = main_part_2();
    Ok(())
}