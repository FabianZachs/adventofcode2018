#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::ops::Range;
use std::str::FromStr;

use std::error::Error;
use std::io::{self, Read, Write};
use std::result;

// ',' is in variable x
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

    let mut events_by_guard = HashMap::<GuardID, Vec<Log>>::new();
    let mut current_guard_id = None;
    // we move logs into hashmap
    for log in logs {
        if let Action::StartShift {
            guard_id: new_guard,
        } = log.action
        {
            current_guard_id = Some(new_guard);
        };

        match current_guard_id {
            Some(id) => events_by_guard.entry(id).or_default().push(log),
            None => return err!("No current guard"),
        }
    }

    let mut guards_sleep_schedule = HashMap::<GuardID, [u32; 60]>::new();
    // get all hashmap values
    // go through each log in Vec<Log>
    for (&guard_id, guard_events) in events_by_guard.iter() {
        let mut sleep_times = [0; 60];
        for sleep_range in MinuteIter::new(guard_events) {
            for minute in sleep_range? {
                sleep_times[minute as usize] += 1;
            }
        }
        guards_sleep_schedule.insert(guard_id, sleep_times);
    }
    part1(&guards_sleep_schedule)?;
    part2(&guards_sleep_schedule)?;

    Ok(())
}

// find guard with most sleeping and
fn part1(guards_sleep_schedule: &HashMap<GuardID, [u32; 60]>) -> Result<()> {
    // find guard that slept the most
    let (&guard_id, sleep_schedule) = guards_sleep_schedule
        .iter()
        .max_by_key(|(_, sleep_schedule)| sleep_schedule.iter().sum::<u32>())
        .unwrap();

    let sleepiest_minute = sleepiest_minute(&sleep_schedule);

    writeln!(io::stdout(), "Part 1: {}", guard_id * sleepiest_minute)?;
    Ok(())
}

struct SleepiestGuard {
    guard_id: GuardID,
    sleepiest_minute: u32,
    time_slept: u32,
}

impl SleepiestGuard {
    fn update(&mut self, new_guard_id: GuardID, new_sleepiest_minute: u32, new_time_slept: u32) {
        if self.time_slept < new_time_slept {
            self.guard_id = new_guard_id;
            self.sleepiest_minute = new_sleepiest_minute;
            self.time_slept = new_time_slept;
        }
    }
    fn new() -> Self {
        SleepiestGuard {
            guard_id: 0,
            sleepiest_minute: 0,
            time_slept: 0,
        }
    }
}

// Of all guards, which guard is most frequently asleep on the same minute?
// What is the ID of the guard you chose multiplied by the minute you chose?
fn part2(guards_sleep_schedule: &HashMap<GuardID, [u32; 60]>) -> Result<()> {
    let mut sleepiest_guard = SleepiestGuard::new();
    for (&guard_id, sleep_schedule) in guards_sleep_schedule {
        let sleepiest_minute = sleepiest_minute(sleep_schedule);
        sleepiest_guard.update(
            guard_id,
            sleepiest_minute,
            sleep_schedule[sleepiest_minute as usize],
        );
    }
    writeln!(
        io::stdout(),
        "Part 2: {}",
        sleepiest_guard.guard_id * sleepiest_guard.sleepiest_minute
    )?;
    Ok(())
}

fn sleepiest_minute(sleep_schedule: &[u32; 60]) -> u32 {
    let (sleepiest_minute, ..) = sleep_schedule
        .iter()
        .enumerate()
        .max_by_key(|(_, freq)| -> u32 { **freq })
        .expect("Iterator of sleepy minutes should not be empty");

    sleepiest_minute as u32
}

type GuardID = u32;

#[derive(Debug)]
enum Action {
    StartShift { guard_id: GuardID },
    FallsAsleep,
    WakesUp,
}

// Order is important here for comparing Date structs
#[derive(Debug, PartialOrd, PartialEq, Eq, Ord)]
struct Date {
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
}

#[derive(Debug)]
struct Log {
    date: Date,
    action: Action,
}

// uses the iter over the Guard's logs
struct MinuteIter<'a> {
    logs: std::slice::Iter<'a, Log>,
}

impl<'a> MinuteIter<'a> {
    fn new(logs: &'a [Log]) -> MinuteIter {
        MinuteIter { logs: logs.iter() }
    }
}

// want to get a range out for each sleep session per next() call i.e. he slept from 20-36
impl<'a> Iterator for MinuteIter<'a> {
    type Item = Result<Range<u32>>;

    fn next(&mut self) -> Option<Result<Range<u32>>> {
        // loop through a FallsAsleep and WakesUp timechunk
        let mut fell_asleep = None;
        loop {
            let log = match self.logs.next() {
                Some(log) => log,
                None => {
                    if fell_asleep.is_some() {
                        return Some(err!(
                            "No matching wake up for falling asleep at end of Guard's logs"
                        ));
                    }
                    return None;
                }
            };
            match log.action {
                Action::StartShift { .. } => {}
                Action::FallsAsleep => fell_asleep = Some(log.date.minute),
                Action::WakesUp => {
                    let minute_fell_asleep = match fell_asleep {
                        None => return Some(err!("Guard woke up without falling asleep")),
                        Some(minute) => minute,
                    };
                    if log.date.minute < minute_fell_asleep {
                        return Some(err!("Guard wokr up before falling asleep"));
                    }
                    //woke_up = Some(log.date.minute);
                    return Some(Ok(fell_asleep.unwrap()..log.date.minute));
                }
            }
        }
    }
}

impl FromStr for Log {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Log> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\[\d{4}-(?P<month>\d{2})-(?P<day>\d{2})\s(?P<hour>\d{2}):(?P<minute>\d{2})]\s(?:Guard\s#(?P<id>\d+)\sbegins\sshift|(?P<action>.*))").unwrap();
        };
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

        let action_type = if let Some(id) = captures.name("id") {
            Action::StartShift {
                guard_id: id.as_str().parse()?,
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
            action: action_type,
        })
    }
}
