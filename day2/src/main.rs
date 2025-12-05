use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn read_file() -> Result<String, io::Error> {

    let mut file = File::open("./day2.1.txt")?;
    let mut input_range_string = String::new();
    file.read_to_string(&mut input_range_string)?;
    Ok(input_range_string)
}

fn exp10(num: u32) -> i64 {
    let base: i64 = 10;
    return base.pow(num);
}
fn main_part1() -> std::io::Result<()> {
    let input_range_string = read_file()?;
    let mut invalid_id_sum: i64 = 0;

    for input_range in input_range_string.split(',') {
        let input_bounds: Vec<&str> = input_range.split('-').collect();
        let lower_bound: i64 = input_bounds[0].parse().expect("This should be a number");
        let upper_bound: i64 = input_bounds[1].parse().expect("This should be a number");

        let lower_digits = lower_bound.ilog10() + 1;
        let mut lower_chunk: i64;
        // Bitwise check if odd
        if (lower_digits & 1) == 1 {
            // While it may seem like you need to add 1, the 10^n thing means that you don't
            lower_chunk = exp10((lower_digits)/2)
        }
        else {
            lower_chunk = lower_bound / exp10((lower_digits)/2);
            if lower_chunk < lower_bound - (exp10(lower_digits/2) * lower_chunk) {
                lower_chunk += 1;
            } 
        }

        let upper_digits = upper_bound.ilog10() + 1;
        let mut upper_chunk: i64;
        // Bitwise check if odd
        if (upper_digits & 1) == 1 {
            // While it may seem like you need to add 1, the 10^n thing means that you don't
            upper_chunk = exp10((upper_digits)/2)
        }
        else {
            upper_chunk = upper_bound / exp10((upper_digits)/2);
            if upper_chunk > upper_bound - (exp10(upper_digits/2) * upper_chunk) {
                upper_chunk -= 1;
            } 
        }
        
        let mut current_chunk = lower_chunk;
        while current_chunk <= upper_chunk {
            let current_num = current_chunk + current_chunk * exp10(current_chunk.ilog10()+1);
            //println!("Chunk: {}, Num: {}", current_chunk, current_num);
            if current_num >= lower_bound && current_num <= upper_bound {
                invalid_id_sum += current_num;
            }
            current_chunk += 1;
        }
        
        //println!("Lower: {}, Digits: {}, lower chunk {}", lower_bound, lower_digits, lower_chunk);
    } 
    print!("{}", invalid_id_sum);
    Ok(())
}

fn check_sized_chunks(num_string: &str, size:usize, n_digits: usize) -> bool {

    //let slots = n_digits / size;
    //println!("Checking size {} with {} slots", size, slots);
    //print!("{}", slots);
    for j in 0..size {
        let test_char: char = num_string.chars().nth(j).expect("j should never be larger than the length of the string");
        let mut iter = num_string.chars().into_iter();
        if j > 0 {
            iter.nth((j - 1).try_into().expect("J should be non-zero and smaller than the length"));
        }
        for c in iter.step_by(size.try_into().expect("size should be a reasonable number to make a usize")) {
            //println!("Baseline: {}, Test: {}", test_char, c);
            if c != test_char {
                //println!("Failed");
                return false
            }
        }
    };

    return true
}

fn check_if_valid(check_num: i64) -> bool{
    let check_num_string = format!("{}", check_num);
    let n_digits = check_num_string.chars().count();

    //println!("Bad debug tool 1");
    for i in 1..(n_digits) {
        
        if n_digits % i == 0 {

            //println!("Bad debug tool 2");
            let valid = check_sized_chunks(&check_num_string, i, n_digits);
            //println!("Valid? {}", valid);
            if valid {
                println!("invalid ID: {}", check_num);
                return true
            }
        }
    }
    return false
}




fn main_part2() -> std::io::Result<()> {
    let input_range_string = read_file()?;
    let mut invalid_id_sum: i64 = 0;
    
    //println!("Invalid ID Sum: {}", invalid_id_sum);
    for input_range in input_range_string.split(',') {
        let input_bounds: Vec<&str> = input_range.split('-').collect();
        let lower_bound: i64 = input_bounds[0].parse().expect("This should be a number");
        let upper_bound: i64 = input_bounds[1].parse().expect("This should be a number");
        let mut current_num: i64 = lower_bound;

        //println!("{}", current_num);
        while current_num <= upper_bound {
            if check_if_valid(current_num) {
                invalid_id_sum += current_num
            }
            current_num += 1;
        }
    }
    println!("Invalid ID Sum: {}", invalid_id_sum);
    Ok(())

}


fn main() -> std::io::Result<()> {
    println!("starting");
    let _ = main_part2()?;
    //let check_num: i64 = 1188511880;
    //let valid = check_if_valid(check_num);
    //println!("Is {} a hit? {}", check_num, valid);
    println!("Done");
    
    Ok(())


}