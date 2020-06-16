#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::str::FromStr;
// Need to sort the logs
// create a vector of logs, make logs comparable, then call vec.sort()

// One guard per night 23:xx-00:59
// HashMap<guirdID, Guard>

// Find guard most asleep
// Find what time they are most asleep
// Answer: GuardID * (minute most asleep)

use std::error::Error;
use std::io::{self, Read};
use std::result;

// ',' is in variable x
//macro_rules! err {
//    ($($tt:tt)*) => { Err(Box::<Error>::from(format!($($tt)*))) }
//}
macro_rules! err {
    ( $($x:tt)* ) => {Err(Box::<dyn Error>::from(format!($($x)*))) }
}

type Result<T> = result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut logs: Vec<Log> = Vec::new();
    for line in input.lines() {
        let log = line
            .parse()
            .or_else(|err| err!("Parsing error: {} with line {:?}", err, line))?;
        logs.push(log);
    }
    logs.sort_by(|a, b| a.date.cmp(&b.date));

    //for l in &logs {
    //    println!("{:#?}", l);
    //}

    Ok(())
}

type GuardID = u32;

#[derive(Debug)]
enum Action {
    StartShift { guardID: GuardID },
    FallsAsleep,
    WakesUp,
}

#[derive(Debug, PartialOrd, PartialEq, Eq, Ord)]
struct Date {
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
}

#[derive(Debug)]
struct Log {
    date: Date,
    action: Action,
}

impl FromStr for Log {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Log> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\[\d{4}-(?P<month>\d{2})-(?P<day>\d{2})\s(?P<hour>\d{2}):(?P<minute>\d{2})]\s(?:Guard\s#(?P<id>\d+)\sbegins\sshift|(?P<action>.*))").unwrap();
        };
        //println!("{}", s);
        let captures = match RE.captures(s) {
            Some(cap) => cap,
            None => {
                return err!("Regex failed to match");
            }
        };

        let date = Date {
            day: captures["day"].parse()?,
            month: captures["month"].parse()?,
            hour: captures["hour"].parse()?,
            minute: captures["minute"].parse()?,
        };

        let actionType = if let Some(id) = captures.name("id") {
            Action::StartShift {
                guardID: id.as_str().parse()?,
            }
        } else if &captures["action"] == "falls asleep" {
            Action::FallsAsleep
        } else if &captures["action"] == "wakes up" {
            Action::WakesUp
        } else {
            return err!("Couldn't find a valid action");
        };

        Ok(Log {
            date,
            action: actionType,
        })
    }
}

