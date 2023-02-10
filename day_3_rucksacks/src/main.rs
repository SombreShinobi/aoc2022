fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    part1()
}

fn part2() -> color_eyre::Result<()> {
    let mut score = 0;

    Ok(())
}

fn part1() -> color_eyre::Result<()> {
    let mut score = 0;

    for sack in include_str!("input.txt").lines() {
        let (first, second) = sack.split_at(sack.len() / 2);

        let first: Vec<Item> = first
            .bytes()
            .map(Item::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        let second: Vec<Item> = second
            .bytes()
            .map(Item::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        'outer: for item in &first {
            for second_item in &second {
                if item == second_item {
                    score += item.priority();
                    break 'outer;
                }
            }
        }

        println!("{first:?} | {second:?}");
    }
    println!("The score is: {score}");

    Ok(())
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
struct Item(u8);

impl std::fmt::Debug for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0 as char)
    }
}

impl TryFrom<u8> for Item {
    type Error = color_eyre::Report;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'a'..=b'z' | b'A'..=b'Z' => Ok(Item(value)),
            _ => Err(color_eyre::eyre::eyre!(
                "{} is not a valid item.",
                value as char
            )),
        }
    }
}

impl Item {
    fn priority(self) -> usize {
        match self {
            Item(b'a'..=b'z') => 1 + (self.0 - b'a') as usize,
            Item(b'A'..=b'Z') => 27 + (self.0 - b'A') as usize,
            _ => unreachable!(),
        }
    }
}
