use std::io::{self, Read, Write};
use std::str::FromStr;

const NX: usize = 300;
const NY: usize = 300;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut locations = input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<Coordinate>>>()?; // this ? mark applies on all elements

    if locations.is_empty() {
        return Err(From::from("No coordinates"));
    }
    //for v in coordinates {
    //    println!("{:#?}", v);
    //}
    let mut grid: [[Option<Coordinate>; NX]; NY] = [[None; NX]; NY];

    for j in (0..NY) {
        for i in (0..NX) {
            grid[j][i] = Some(find_nearest_location(i, j, &locations));
        }
    }
    part1(&locations)?;

    Ok(()) test
}

fn find_nearest_location(i: usize, j: usize, locations: &[Coordinate]) -> Coordinate {
    //for location in locations {}
    Coordinate { x: 0, y: 0 }
}

fn part1(coordinates: &[Coordinate]) -> Result<()> {
    Ok(())
}

#[derive(Debug, Clone, Copy)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn manhattan_distance_from(&self, other: &Coordinate) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl FromStr for Coordinate {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Coordinate> {
        let mut line_iter = match s.find(",") {
            None => return Err(From::from("No comma found")),
            Some(_) => s.split(", "),
        };

        let x = line_iter.next().unwrap().parse()?;
        let y = line_iter.next().unwrap().parse()?;
        Ok(Coordinate { x, y })
    }
}
