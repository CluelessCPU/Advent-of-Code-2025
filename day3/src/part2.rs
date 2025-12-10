

pub fn get_max_line_joltage(line_str: &str, n_batteries: usize) -> Result<i64, &str> {
    let mut line_digits: Vec<i64> = Vec::new();
    let mut sum_digits: Vec<i64> = Vec::new();
    let mut joltage_sum: i64 = 0;

    // Convert string to vec of digits
    for c in line_str.chars() {
        let result = c.to_digit(10);
        match result {
            Some(x)     => line_digits.push(x.try_into().expect("A digit between 1 and 9 should always fit into an i32")),
            
            None        => return Err("All chars in line_str must map to a decimal digit"),
        }
    } 

    // Get the n_batteries top digits
    let line_length = line_digits.len();

    let mut prev_position: usize = 0;
    for i in 0..n_batteries {
        //println!("i={}, prev={}", i, prev_position);
        let mut current_position = 0;
        let iter = line_digits[prev_position..=(line_length-(n_batteries-i))]
            .into_iter()
            .enumerate()
            .rev();


        let mut max_value = 0;
        for tup in iter {
            if *tup.1 >= max_value {
                max_value = *tup.1;
                current_position = tup.0 + prev_position;
            }
            //println!("Val: {}, position: {}, Max {}", *tup.1, tup.0, max_value);
        } 
        sum_digits.push(max_value);
        prev_position = current_position + 1;

    }

    let base:i64 = 10;
    let mut expn = n_batteries;
    for n in &sum_digits {
        expn -= 1; 
        joltage_sum += base.pow(expn.try_into().expect("expn should always be a small positive number")) * n;
    }



    return Ok(joltage_sum);

}
