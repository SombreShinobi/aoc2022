fn main() -> color_eyre::Result<()>{
    color_eyre::install()?;

    part_1()
}

fn part_1() -> color_eyre::Result<()> {
    Ok(())
}

struct Stack {
    crates: Vec<&str>,
}

impl Stack {
    type Err = color_eyre::Report;

    fn stack_crate(&mut self, string: &str) -> Result<Self, Self::Err> {
        let chars = string.chars();
        let (Some(), Some(crate), Some(), None) = (chars.next(), chars.next(), chars.next(), chars.next()) else {
            return Err(color_eyre::eyre::eyre!("Not a valid crate input!"));
        };
    }
}

struct Ship {
    stack1: Stack,
    stack2: Stack,
    stack3: Stack,
    stack4: Stack,
    stack5: Stack,
    stack6: Stack,
    stack7: Stack,
    stack8: Stack,
    stack9: Stack,
}
