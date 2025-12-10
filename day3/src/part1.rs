
pub fn get_max_line_joltage(line_str: &str) -> Result<i32, &str> {
    let mut prev_num: i32 = 0;
    let mut max_leader: i32 = 0;
    let mut max_follower: i32= 0;
    
    for c in line_str.chars() {
        let current_num: i32;
        let result = c.to_digit(10);
        match result {
            Some(x)     => current_num = x.try_into().expect("A digit between 1 and 9 should always fit into an i32"),
            
            None        => return Err("All chars in line_str must map to a decimal digit"),
        }
        
        if prev_num > max_leader {
            max_leader = prev_num;
            max_follower = current_num;
        } else if current_num > max_follower {
            max_follower = current_num
        }
        //println!("Leader: {}, Follower {}", max_leader, max_follower);
        prev_num = current_num;
    }
    let max_joltage: i32 = max_leader * 10 + max_follower;

    return Ok(max_joltage);
}