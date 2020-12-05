//! --- Day 4: Repose Record ---
//!
//! You've sneaked into another supply closet - this time, it's across from the prototype
//! suit manufacturing lab. You need to sneak inside and fix the issues with the suit,
//! but there's a guard stationed outside the lab, so this is as close as you can safely get.
//!
//! As you search the closet for anything that might help, you discover that you're
//! not the first person to want to sneak in. Covering the walls, someone has spent
//! an hour starting every midnight for the past few months secretly observing this
//! guard post! They've been writing down the ID of the one guard on duty that night -
//! the Elves seem to have decided that one guard was enough for the overnight shift -
//! as well as when they fall asleep or wake up while at their post (your puzzle input).
//!
//! For example, consider the following records, which have already been organized
//! into chronological order:
//!
//! [1518-11-01 00:00] Guard #10 begins shift
//! [1518-11-01 00:05] falls asleep
//! [1518-11-01 00:25] wakes up
//! [1518-11-01 00:30] falls asleep
//! [1518-11-01 00:55] wakes up
//! [1518-11-01 23:58] Guard #99 begins shift
//! [1518-11-02 00:40] falls asleep
//! [1518-11-02 00:50] wakes up
//! [1518-11-03 00:05] Guard #10 begins shift
//! [1518-11-03 00:24] falls asleep
//! [1518-11-03 00:29] wakes up
//! [1518-11-04 00:02] Guard #99 begins shift
//! [1518-11-04 00:36] falls asleep
//! [1518-11-04 00:46] wakes up
//! [1518-11-05 00:03] Guard #99 begins shift
//! [1518-11-05 00:45] falls asleep
//! [1518-11-05 00:55] wakes up
//!
//! Timestamps are written using year-month-day hour:minute format. The guard falling
//! asleep or waking up is always the one whose shift most recently started.
//! Because all asleep/awake times are during the midnight hour (00:00 - 00:59),
//! only the minute portion (00 - 59) is relevant for those events.
//!
//! Visually, these records show that the guards are asleep at these times:
//!
//! Date   ID   Minute
//!             000000000011111111112222222222333333333344444444445555555555
//!             012345678901234567890123456789012345678901234567890123456789
//! 11-01  #10  .....####################.....#########################.....
//! 11-02  #99  ........................................##########..........
//! 11-03  #10  ........................#####...............................
//! 11-04  #99  ....................................##########..............
//! 11-05  #99  .............................................##########.....
//!
//! The columns are Date, which shows the month-day portion of the relevant day; ID,
//! which shows the guard on duty that day; and Minute, which shows the minutes during
//! which the guard was asleep within the midnight hour.
//! (The Minute column's header shows the minute's ten's digit in the first row and
//! the one's digit in the second row.)
//! Awake is shown as ., and asleep is shown as #.
//!
//! Note that guards count as asleep on the minute they fall asleep, and they count as
//! awake on the minute they wake up. For example, because Guard #10 wakes up at
//! 00:25 on 1518-11-01, minute 25 is marked as awake.
//!
//! If you can figure out the guard most likely to be asleep at a specific time, you
//! might be able to trick that guard into working tonight so you can have the best
//! chance of sneaking in. You have two strategies for choosing the best guard/minute combination.
//!
//! Strategy 1: Find the guard that has the most minutes asleep. What minute does
//! that guard spend asleep the most?
//!
//! In the example above, Guard #10 spent the most minutes asleep, a total of 50
//! minutes (20+25+5), while Guard #99 only slept for a total of 30 minutes (10+10+10).
//! Guard #10 was asleep most during minute 24 (on two days, whereas any other minute the
//! guard was asleep was only seen on one day).
//!
//! While this example listed the entries in chronological order, your entries are in the
//! order you found them. You'll need to organize them before they can be analyzed.
//!
//! What is the ID of the guard you chose multiplied by the minute you chose?
//! (In the above example, the answer would be 10 * 24 = 240.)

use std::str::FromStr;

pub fn solve() -> crate::Result<()> {
    let input = crate::read_input("day04.txt")?;
    println!("Day04 part1: {}", part1(&input));

    Ok(())
}

fn part1(s: &str) -> usize {
    let log = parse(s);

    let mut guard_reports = Vec::new();

    let mut active_report = None;
    let mut fell_asleep = None;

    for r in log {
        match r.action {
            Action::BeginsShift { id } => {
                if let Some(prev_report) = active_report.replace(GuardReport {
                    guard_id: id,
                    longest_sleep: 0,
                }) {
                    guard_reports.push(prev_report);
                }

                fell_asleep = None;
            }
            Action::FallsAsleep => {
                fell_asleep = Some(r.ts);
            }

            Action::WakesUp => {
                if let Some(ref mut active_report) = active_report {
                    let sleep_duration = fell_asleep
                        .map(|ref ts| r.ts.duration_since(ts))
                        .unwrap_or_default()
                        .minutes;

                    if active_report.longest_sleep < sleep_duration {
                        active_report.longest_sleep = sleep_duration;
                    }
                }
            }
        }
    }

    1
}

fn parse(s: &str) -> Vec<LogRow> {
    let mut log: Vec<LogRow> = s
        .trim()
        .lines()
        .map(|s| s.trim())
        .map(|s| s.parse::<LogRow>())
        .collect::<Result<_, _>>()
        .expect("Parsing row");

    log.sort_by(|r1, r2| r1.ts.cmp(&r2.ts));
    log
}

#[derive(
    Debug,
    Clone,
    Copy,
    parse_display::FromStr,
    parse_display::Display,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
)]
#[display("[{year}-{month}-{day} {hour}:{min}]")]
struct Ts {
    year: usize,
    month: usize,
    day: usize,
    hour: usize,
    min: usize,
}

impl Ts {
    fn duration_since(&self, other: &Ts) -> Duration {
        assert_eq!(self.year, other.year);
        assert_eq!(self.month, other.month);

        let day_diff = self.day - other.day;
        let hour_diff = self.hour - other.hour;
        let min_diff = self.min - other.min;

        Duration {
            minutes: day_diff * 24 * 60 + hour_diff * 60 + min_diff,
        }
    }
}
#[derive(Clone, Copy, Default)]
struct Duration {
    minutes: usize,
}

#[derive(Debug, parse_display::Display)]
#[display("[{ts.year:04}-{ts.month:02}-{ts.day:02} {ts.hour:02}:{ts.min:02}] {action}")]
struct LogRow {
    ts: Ts,
    action: Action,
}

impl FromStr for LogRow {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let i = s
            .find(']')
            .ok_or_else(|| format!("Could not find closing `]` in row `{}`", s))?;
        let (ts_str, action_str) = s.split_at(i + 1);

        let ts = ts_str
            .parse()
            .map_err(|err| format!("Invalid ts `{}`: {}", ts_str, err))?;

        let action = action_str
            .trim()
            .parse()
            .map_err(|err| format!("Invalid action `{}`: {}", action_str, err))?;

        Ok(Self { ts, action })
    }
}

#[derive(Debug, parse_display::FromStr, parse_display::Display)]
enum Action {
    #[display("Guard #{id} begins shift")]
    BeginsShift { id: usize },
    #[display("falls asleep")]
    FallsAsleep,
    #[display("wakes up")]
    WakesUp,
}

struct GuardReport {
    guard_id: usize,
    longest_sleep: usize,
}

#[cfg(test)]
mod tests {

    #[test]
    fn part1_example() {
        let input = r#"
[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up
"#
        .trim();

        let v = super::parse(input);

        v.iter().for_each(|row| println!("{}", row));
        assert!(false);
    }
}
