// TODO I think my iterator is slightly wrong. Part 2 gives 2 answers
#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::error::Error;
use std::io::{self, Read, Write};
use std::str::FromStr;

use regex::Regex;

const NX: usize = 1000;
const NY: usize = 1000;

// alias
//type Result<T> = result::Result<T, String>;
//type Result<T> = result::Result<T, Box<Error>>;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let input = input.trim();

    let mut claims: Vec<Claim> = Vec::with_capacity(1293);
    for line in input.lines() {
        let claim = line.parse().or_else(|err| {
            Err(Box::<dyn Error>::from(format!(
                "Parsing: {:?}, error {}",
                line, err
            )))
        })?;
        claims.push(claim);
    }

    let mut grid = [0u8; NX * NY];
    //for claim in &claims {
    //    for (x, y) in claim.iter_coordinates() {
    //        grid[x + NX * y] += 1;
    //    }
    //}

    for claim in &claims {
        for i in 0..claim.width {
            for j in 0..claim.height {
                grid[(claim.x + i) + NX * (claim.y + j)] += 1;
            }
        }
    }

    part1(&grid)?;
    part2(&claims, &grid)?;

    Ok(())
}

fn part1(grid: &[u8]) -> Result<(), Box<dyn Error>> {
    // iter() creates iter, thus &
    // filter() creates iter, thus &&
    let count = grid.iter().filter(|&&x| x > 1).count();
    writeln!(io::stdout(), "Part 1: {}", count)?;

    Ok(())
}

fn part2(claims: &[Claim], grid: &[u8]) -> Result<(), Box<dyn Error>> {
    for claim in claims {
        if claim.iter_coordinates().all(|(x, y)| grid[x + y * NX] == 1) {
            writeln!(io::stdout(), "Part 2: {}", claim.id)?;
        }
    }
    Ok(())
}

#[derive(Debug)]
struct Claim {
    id: u16,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

// we can get an iterator from a claim
impl Claim {
    fn iter_coordinates(&self) -> IterCoordinates {
        IterCoordinates {
            claim: &self,
            px: self.x,
            py: self.y,
        }
    }
}

struct IterCoordinates<'a> {
    claim: &'a Claim,
    px: usize,
    py: usize,
}

impl<'a> Iterator for IterCoordinates<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<(usize, usize)> {
        // Case: at width boundary -> py++, px = 0
        if self.px >= self.claim.x + self.claim.width - 1 {
            self.px = self.claim.x;
            self.py += 1;
        } else {
            // Case: at neither boundary -> px++
            self.px += 1;
        }
        // Case: at width and height boundary -> None
        match self {
            IterCoordinates { px, py, .. }
                if *px < self.claim.x + self.claim.width
                    && *py < self.claim.y + self.claim.height =>
            {
                Some((*px, *py))
            }
            _ => None,
        }
    }
}

// to allow parsing str into Claim
impl FromStr for Claim {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Claim, Box<dyn Error>> {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(
                r"#(?P<id>[0-9]+)\s@\s(?P<x>[0-9]+),(?P<y>[0-9]+):\s(?P<width>[0-9]+)x(?P<height>[0-9]+)"
            )
            .unwrap();
        }
        let capture = match REGEX.captures(s) {
            Some(captures) => captures,
            None => return Err(Box::<dyn Error>::from("Regex error")),
        };

        // capture["id"] returns str
        // capture.get(0) gets Options
        Ok(Claim {
            id: capture["id"].parse()?,
            x: capture["x"].parse()?,
            y: capture["y"].parse()?,
            width: capture["width"].parse()?,
            height: capture["height"].parse()?,
        })
    }
}
