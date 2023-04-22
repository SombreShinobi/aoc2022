use std::collections::VecDeque;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    // part_1()
    part_2()
}

fn part_1() -> color_eyre::Result<()> {
    let mut ship = Ship::new();
    for row in include_str!("crates.txt").lines().to_owned() {
        ship.stack_row(parse_line(row).unwrap());
    }

    for command in include_str!("moves.txt").lines() {
        let (amount, from, to) = parse_command(command).unwrap();

        for _ in 0..amount {
            ship.move_crate(from - 1, to - 1);
        }
    }

    println!("{}", ship.get_top());

    Ok(())
}

fn part_2() -> color_eyre::Result<()> {
    let mut ship = Ship::new();
    for row in include_str!("crates.txt").lines() {
        ship.stack_row(parse_line(row).unwrap());
    }

    for command in include_str!("moves.txt").lines() {
        let (amount, from, to) = parse_command(command).unwrap();

        ship.move_crates(amount, from - 1, to - 1);
    }

    println!("{}", ship.get_top());

    Ok(())
}

fn parse_line(line: &str) -> color_eyre::Result<String> {
    let mut result = String::new();
    let mut i = 3;
    while i < line.len() {
        result.push(line.chars().nth(i).unwrap());
        i += 4;
    }

    Ok(result)
}

fn parse_command(line: &str) -> color_eyre::Result<(usize, usize, usize)> {
    let result = line
        .replace(&['m', 'o', 'v', 'e', 'f', 'r', 'o', 'm', 't'], "")
        .to_owned();

    let mut commands = result.split(" ").into_iter();
    if let (Some(_), Some(amount), Some(_), Some(from), Some(_), Some(to)) = (
        commands.next(),
        commands.next(),
        commands.next(),
        commands.next(),
        commands.next(),
        commands.next(),
    ) {
        Ok((
            amount.parse().unwrap(),
            from.parse().unwrap(),
            to.parse().unwrap(),
        ))
    } else {
        Err(color_eyre::eyre::eyre!("Error getting command strings"))
    }
}

#[derive(Clone, Debug)]
struct Stack {
    crates: VecDeque<char>,
}

impl Stack {
    fn new() -> Self {
        Self {
            crates: VecDeque::new(),
        }
    }

    fn stack_crate(&mut self, name: char) -> color_eyre::Result<()> {
        if name != " ".chars().next().unwrap() {
            self.crates.push_back(name);
        }

        Ok(())
    }

    fn remove_crate(&mut self) -> color_eyre::Result<char> {
        let name = match self.crates.pop_front() {
            Some(name) => name,
            None => {
                return Err(color_eyre::eyre::eyre!(
                    "You cannot remove a crate which doesn't exist!"
                ))
            }
        };

        Ok(name)
    }
}

#[derive(Debug)]
struct Ship {
    stacks: Vec<Stack>,
}

impl Ship {
    fn new() -> Self {
        Self {
            stacks: vec![Stack::new(); 9],
        }
    }

    fn stack_row(&mut self, row: String) -> color_eyre::Result<()> {
        for (i, char) in row.chars().enumerate() {
            self.stacks[i].stack_crate(char).unwrap();
        }

        Ok(())
    }

    fn move_crate(&mut self, from: usize, to: usize) -> color_eyre::Result<()> {
        let name = self.stacks[from].remove_crate().unwrap();
        self.stacks[to].crates.push_front(name);
        Ok(())
    }

    fn move_crates(&mut self, amount: usize, from: usize, to: usize) -> color_eyre::Result<()> {
        let mut removed_crates: Vec<char> = Vec::new();
        for _ in 0..amount {
            removed_crates.push(self.stacks[from].remove_crate().unwrap());
        }

        for crate_name in removed_crates.into_iter().rev() {
            self.stacks[to].crates.push_front(crate_name);
        }

        Ok(())
    }

    fn get_top(&self) -> String {
        let mut result = String::new();
        for stack in &self.stacks {
            result.push(stack.crates[0]);
        }

        result
    }
}
