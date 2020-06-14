use std::collections::HashSet;
use std::io::{self, Read, Result, Write};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(input.trim())?;
    part2(input.trim())?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let sum: i32 = input
        .lines()
        .map(|line| line.parse::<i32>().expect("parsing problem"))
        .fold(0, |acc, x| acc + x);

    writeln!(io::stdout(), "Part 1: {}", sum)?;
    Ok(())
}

// use cycle on iterator
fn part2(input: &str) -> Result<()> {
    let mut set = HashSet::new();
    set.insert(0);

    let mut net_frequency = 0;
    loop {
        for freq in input.lines().map(|line| line.parse::<i32>().unwrap()) {
            net_frequency += freq;
            if (set.contains(&net_frequency)) {
                writeln!(io::stdout(), "Part 2: {}", net_frequency)?;
                return Ok(());
            }
            set.insert(net_frequency);
        }
    }
    Ok(())
}
