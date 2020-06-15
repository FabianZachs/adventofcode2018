use std::io::{self, Read, Result, Write};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let input = input.trim();

    part1(input);
    part2(input);

    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let mut occurrences: [u8; 256] = [0; 256]; // 256 for all ASCII values
    let (mut two_counts, mut three_counts) = (0, 0);

    for line in input.lines() {
        occurrences.iter_mut().for_each(|x| *x = 0);
        for c in line.as_bytes().iter().map(|&x| x as usize) {
            occurrences[c] += 1;
        }

        if occurrences.iter().any(|x| *x == 2) {
            two_counts += 1;
        }
        if occurrences.iter().any(|x| *x == 3) {
            three_counts += 1;
        }
    }

    writeln!(io::stdout(), "Part 1: {}", two_counts * three_counts)?;
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let ids: Vec<_> = input.lines().collect();
    for i in 0..ids.len() {
        for j in i + 1..ids.len() {
            if hamming_distance(&ids[i], &ids[j]) == 1 {
                writeln!(io::stdout(), "Part 2: {}", solution(&ids[i], &ids[j]))?;
                return Ok(());
            }
        }
    }
    Ok(())
}

fn hamming_distance(x: &str, y: &str) -> u8 {
    //let y = y.as_bytes().iter();
    let iter = x.as_bytes().iter().zip(y.as_bytes().iter());
    let mut distance = 0;
    for (x1, x2) in iter {
        if x1 != x2 {
            distance += 1;
        }
    }
    distance
}

fn solution(x: &str, y: &str) -> String {
    x.chars()
        //.iter()
        .zip(y.chars())
        .filter(|&(c1, c2)| c1 == c2)
        .map(|(c1, _)| c1)
        .collect()
}
