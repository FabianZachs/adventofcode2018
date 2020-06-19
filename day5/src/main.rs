use std::io::{self, Read, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
// polymers with this difference react
const UNITS_DIFFERENCE: u8 = 32;
// identifier for destroyed polymer
const DESTROYED_UNIT: u8 = 0;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    //writeln!(io::stdout(), "HELLO")?;
    let input = input.trim();
    let mut polymer: Vec<u8> = input.bytes().collect();
    //part1(&mut polymer)?;
    part1(&mut polymer.clone())?;
    part2(&input)?;
    Ok(())
}

// not sure why polymer has to be mutable as well as the reference
fn part1(mut polymer: &mut [u8]) -> Result<()> {
    let length = react_polymer(&mut polymer)?;
    writeln!(io::stdout(), "Part 1: {}", length)?;

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut shortest_length = std::u32::MAX;
    for i in b'A'..b'Z' {
        let mut polymer: Vec<u8> = input
            .replace(i as char, "")
            .replace((i + UNITS_DIFFERENCE) as char, "")
            .bytes()
            .collect();
        let length = react_polymer(&mut polymer)?;
        if length < shortest_length {
            shortest_length = length;
        }
    }
    writeln!(io::stdout(), "Part 2: {}", shortest_length)?;
    Ok(())
}

fn react_polymer(mut polymer: &mut [u8]) -> Result<u32> {
    let mut lower: usize = 0;
    let mut higher: usize = 1;

    while higher < polymer.len() {
        if isPolar(polymer[lower], polymer[higher]) {
            destroyPolymers(lower, higher, &mut polymer);
            lower = findPrecedingPolymerIndex(lower, &polymer);
            higher = findNextPolymerIndex(higher, &polymer);
        } else {
            lower = findNextPolymerIndex(lower, &polymer);
            higher = findNextPolymerIndex(higher, &polymer);
        }
    }
    let length = polymer.iter().filter(|&&x| x > 0).count();

    Ok(length as u32)
}

fn destroyPolymers(i: usize, j: usize, polymer: &mut [u8]) {
    polymer[i] = DESTROYED_UNIT;
    polymer[j] = DESTROYED_UNIT;
}

fn findNextPolymerIndex(mut i: usize, polymer: &[u8]) -> usize {
    if i == polymer.len() - 1 {
        return i + 1; // since we cant query polymer[polymer.len()]
    }
    while i < polymer.len() {
        i += 1;
        if polymer[i] != DESTROYED_UNIT {
            break;
        }
    }
    i
}

fn findPrecedingPolymerIndex(mut i: usize, polymer: &[u8]) -> usize {
    while i > 0 {
        i -= 1;
        if polymer[i] != DESTROYED_UNIT {
            break;
        }
    }
    i
}

fn isPolar(c1: u8, c2: u8) -> bool {
    if c1 < c2 {
        return c2 - c1 == UNITS_DIFFERENCE;
    }
    return c1 - c2 == UNITS_DIFFERENCE;
}
