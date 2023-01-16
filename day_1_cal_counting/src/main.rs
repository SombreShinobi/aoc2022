use std::io;

fn main() {
    part_1();
    // part_2();
}

fn part_1() {
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

fn part_2() {
    println!("Hello. How many calories are each of you carrying?");
    let mut calories = vec![];
    let mut temp_val = 0;
    let mut buffer = String::new();
    let stdin = io::stdin();
    while stdin.read_line(&mut buffer).is_ok() {
        let trimmed = buffer.trim_end();
        if trimmed == "done" {
            println!(
                "The total number of calories carried by the top three elves is {}",
                calories[0] + calories[1] + calories[2]
            );
            return;
        } else if trimmed == "" {
            calories.push(temp_val);
            calories.sort_by(|a, b| b.cmp(a));
            temp_val = 0;
        } else {
            temp_val += trimmed.parse::<u32>().unwrap();
        }
        buffer.clear();
    }
}
