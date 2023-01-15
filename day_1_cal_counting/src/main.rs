use std::io;

fn main() {
    println!("Hello. How many calories are each of you carrying?");
    let mut highest_val = 0;
    let mut temp_val = 0;
    let mut buffer = String::new();
    let stdin = io::stdin();
    while stdin.read_line(&mut buffer).is_ok() {
        let trimmed = buffer.trim_end();
        if trimmed == "done" {
            println!("The highest number of calories carried by an elf is {highest_val}");
            return;
        } else if trimmed == "" {
            if highest_val < temp_val {
                highest_val = temp_val;
            }
            temp_val = 0;
        } else {
            temp_val += trimmed.parse::<u32>().unwrap();
        }
        buffer.clear();
    }
}
