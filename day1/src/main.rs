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
    input
        .lines()
        .cycle()
        .map(|line| line.parse::<i32>().expect("parsing problem"))
        .for_each(|x| {
            if set.get(&x).is_some() {
                println!("{}", x);
                panic!();
            } else {
                set.insert(x);
            }
        });
    Ok(())
}
