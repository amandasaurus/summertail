extern crate anyhow;
extern crate chrono;
extern crate humantime;
extern crate regex;

use regex::Regex;
use std::cmp::{max, min};
use std::io::{BufRead, BufReader, Write};
use std::sync::mpsc::{sync_channel, RecvTimeoutError};
use std::thread;

use anyhow::Result;

const BARS: [&str; 10] = [
    // User doesn't want bars
    "",
    "   ",
    " \u{2581}\u{258F}",
    " \u{2582}\u{258E}",
    " \u{2583}\u{258D}",
    " \u{2584}\u{258C}",
    " \u{2585}\u{258B}",
    " \u{2586}\u{258A}",
    " \u{2587}\u{2589}",
    " \u{2588}\u{2588}",
];

const CLOCKS: [&str; 13] = [
    "\u{1F55B}\u{fe0e}",
    "\u{1F550}\u{fe0e}",
    "\u{1F551}\u{fe0e}",
    "\u{1F552}\u{fe0e}",
    "\u{1F553}\u{fe0e}",
    "\u{1F554}\u{fe0e}",
    "\u{1F555}\u{fe0e}",
    "\u{1F556}\u{fe0e}",
    "\u{1F557}\u{fe0e}",
    "\u{1F558}\u{fe0e}",
    "\u{1F559}\u{fe0e}",
    "\u{1F55A}\u{fe0e}",
    "\u{1F55B}\u{fe0e}",
];

fn fraction_bar(value: usize, total: usize) -> &'static str {
    let index = if total == 0 {
        0
    } else if value >= total {
        8
    } else {
        ((value * 8) as f64 / total as f64).ceil() as usize
    };

    BARS[index + 1]
}

fn fraction_clock(fraction: f64) -> &'static str {
    CLOCKS[(fraction * 12.).round() as usize]
}

fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();

    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if args.len() < 2 {
        eprintln!("Usage: {} DELAY REGEX1 REGEX2 ...", std::env::args().nth(0).unwrap());
        eprintln!("Source code: {}", option_env!("CARGO_PKG_REPOSITORY").unwrap_or("non-cargo compilation, no CARGO_PKG_REPOSITORY"));
        return Ok(());
    }
    let (delay, args) = args.split_at(1);
    let delay_str = &delay[0];

    let delay = if let Ok(secs) = delay_str.parse::<f32>() {
        chrono::Duration::milliseconds((secs * 1000.).round() as i64)
    } else {
        chrono::Duration::from_std(humantime::parse_duration(&delay_str)?)?
    };

    let wait_between_prints = min(
        delay.to_std()?.mul_f32(0.5),
        std::time::Duration::from_secs(1),
    );

    let labels = args;
    let patterns = labels
        .iter()
        .map(|p| {
            if let Some(rest) = p.strip_prefix('!') {
                Regex::new(rest).map(|r| (r, true))
            } else {
                Regex::new(p).map(|r| (r, false))
            }
        })
        .collect::<Result<Vec<_>, _>>()
        .expect("Invalid regex(es)");

    let mut counters = vec![0; labels.len() + 1];

    let mut num_lines_this_period = 0;
    let mut max_num_lines_this_period = 0;

    let timestamp_format = "%H:%M:%S";

    let mut have_matched_once;
    let mut time_of_last_newline = chrono::Utc::now();
    let mut max_width_numbers = 0;
    let mut fraction_through_delay;

    stdout.flush().unwrap();

    let mut now_utc;
    let mut now_local;

    // Channel for send the stdin lines around
    let (tx, rx) = sync_channel(1000);

    // Start a thread to read from stdin & send to the channel
    thread::spawn(move || {
        let mut bytes = vec![];
        let mut stdin = BufReader::new(stdin);
        while stdin.read_until('\x0A' as u8, &mut bytes).unwrap() > 0 {
            let line = String::from_utf8_lossy(&bytes);
            tx.send(line.to_string()).unwrap();
            bytes.clear();
        }
    });

    // Main loop
    loop {
        let line = match rx.recv_timeout(wait_between_prints) {
            Err(RecvTimeoutError::Disconnected) => {
                // we've finished
                break;
            }
            Err(RecvTimeoutError::Timeout) => None,
            Ok(line) => Some(line),
        };

        now_utc = chrono::Utc::now();
        now_local = chrono::offset::Local::now();

        if now_utc - time_of_last_newline >= delay {
            write!(stdout, "\n").unwrap();

            // clear counters
            for r in counters.iter_mut() {
                *r = 0;
            }
            num_lines_this_period = 0;

            time_of_last_newline = now_utc;
        }
        fraction_through_delay = (now_utc - time_of_last_newline).num_milliseconds() as f64
            / delay.num_milliseconds() as f64;

        have_matched_once = false;
        if let Some(line) = line {
            num_lines_this_period += 1;
            max_num_lines_this_period = max(max_num_lines_this_period, num_lines_this_period);
            for ((ref regex, inv), ref mut result) in patterns.iter().zip(counters.iter_mut()) {
                if inv ^ regex.is_match(&line) {
                    **result += 1;
                    have_matched_once = true;
                }
            }
            if !have_matched_once {
                counters[labels.len()] += 1;
            }
        }

        max_width_numbers = max(
            max_width_numbers,
            (num_lines_this_period as f32).log10().ceil() as usize,
        );

        write!(
            stdout,
            "\r{clock}{time} {num_lines:>max_width_numbers$}{bar}  ",
            clock = fraction_clock(fraction_through_delay),
            time = now_local.format(timestamp_format),
            num_lines = num_lines_this_period,
            max_width_numbers = max_width_numbers,
            // When we're a fraction of the way through, the bar for 'total this period, should
            // be based on
            bar = fraction_bar(
                num_lines_this_period,
                (max_num_lines_this_period as f64 * fraction_through_delay).round() as usize
            ),
        )
        .unwrap();
        for (label, result) in labels.iter().zip(counters.iter()) {
            write!(
                stdout,
                "{label}: {total:>max_width_numbers$} {percent:>6}{bar}  ",
                label = label,
                total = result,
                max_width_numbers = max_width_numbers,
                percent = (if num_lines_this_period == 0 {
                    "".to_string()
                } else {
                    format!(
                        "({:>3.0}%)",
                        ((*result as f32 / num_lines_this_period as f32) * 100.).round()
                    )
                }),
                bar = fraction_bar(*result, num_lines_this_period),
            )
            .unwrap();
        }

        write!(
            stdout,
            "other: {total:>max_width_numbers$} {percent:>6} {bar} ",
            total = counters[labels.len()],
            max_width_numbers = max_width_numbers,
            percent = (if num_lines_this_period == 0 {
                "".to_string()
            } else {
                format!(
                    "({:>3.0}%)",
                    ((counters[labels.len()] as f32 / num_lines_this_period as f32) * 100.).round()
                )
            }),
            bar = fraction_bar(counters[labels.len()], num_lines_this_period),
        )
        .unwrap();

        stdout.flush().unwrap();
    }

    write!(stdout, "\n").unwrap();
    stdout.flush().unwrap();

    Ok(())
}
