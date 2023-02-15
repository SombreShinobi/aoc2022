use std::str::FromStr;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    part_1()?;
    part_2()
}

fn part_1() -> color_eyre::Result<()> {
    let mut containing_pairs = 0;

    for pair in include_str!("input.txt").lines() {
        let pair = Pair::from_str(pair)?;

        if pair.contains() {
            containing_pairs += 1;
        }
    }

    println!("The number of containing pairs is {containing_pairs}");

    Ok(())
}

fn part_2() -> color_eyre::Result<()> {
    let mut containing_pairs = 0;

    for pair in include_str!("input.txt").lines() {
        let pair = Pair::from_str(pair)?;

        if pair.overlaps() {
            containing_pairs += 1;
        }
    }

    println!("The number of overlapping pairs is {containing_pairs}");

    Ok(())
}

struct Assignment {
    start_section: u32,
    end_section: u32,
}

impl FromStr for Assignment {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sections = s.split("-");

        let (Some(start_section), Some(end_section), None) = (sections.next(), sections.next(), sections.next()) else {
            return Err(color_eyre::eyre::eyre!("Section is not a number"));
        };

        let start_section = match start_section.parse() {
            Ok(num) => num,
            Err(_) => return Err(color_eyre::eyre::eyre!("The given input cannot be parsed.")),
        };
        let end_section = match end_section.parse() {
            Ok(num) => num,
            Err(_) => return Err(color_eyre::eyre::eyre!("The given input cannot be parsed.")),
        };

        Ok(Self {
            start_section,
            end_section,
        })
    }
}

struct Pair {
    elf1: Assignment,
    elf2: Assignment,
}

impl FromStr for Pair {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pair = s.split(',');

        let (Some(elf1), Some(elf2), None) = (pair.next(), pair.next(), pair.next()) else {
            return Err(color_eyre::eyre::eyre!("There are not only two pairs!"));
        };

        let elf1 = match Assignment::from_str(elf1) {
            Ok(elf) => elf,
            Err(_) => return Err(color_eyre::eyre::eyre!("The given input cannot be parsed.")),
        };
        let elf2 = match Assignment::from_str(elf2) {
            Ok(elf) => elf,
            Err(_) => return Err(color_eyre::eyre::eyre!("The given input cannot be parsed.")),
        };

        Ok(Self { elf1, elf2 })
    }
}

impl Pair {
    fn contains(&self) -> bool {
        (self.elf1.start_section <= self.elf2.start_section
            && self.elf1.end_section >= self.elf2.end_section)
            || (self.elf2.start_section <= self.elf1.start_section
                && self.elf2.end_section >= self.elf1.end_section)
    }
}

impl Pair {
    fn overlaps(&self) -> bool {
        ((self.elf1.start_section <= self.elf2.start_section
            && self.elf2.start_section <= self.elf1.end_section)
            || (self.elf1.start_section <= self.elf2.end_section
                && self.elf2.end_section <= self.elf1.end_section))
            || ((self.elf2.start_section <= self.elf1.start_section
                && self.elf1.start_section <= self.elf2.start_section)
                || (self.elf2.start_section <= self.elf1.end_section
                    && self.elf1.end_section <= self.elf2.end_section))
    }
}
