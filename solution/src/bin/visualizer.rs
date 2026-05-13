//! Filler visualizer.
//!
//! Reads the verbose output of `game_engine` from stdin (or a file) and re-renders
//! each board frame with ANSI colors, with a configurable delay between frames.
//!
//! Examples:
//!   ./game_engine -f maps/map00 -p1 robots/wall_e -p2 ./filler | ./visualizer
//!   ./game_engine -f maps/map00 -p1 robots/wall_e -p2 ./filler > game.log
//!   ./visualizer game.log --delay 200

use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::thread::sleep;
use std::time::Duration;

const CLEAR: &str = "\x1b[2J\x1b[H";
const RESET: &str = "\x1b[0m";
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const CYAN: &str = "\x1b[36m";
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";

fn colorize(c: char) -> String {
    match c {
        '@' => format!("{}{}@{}", BOLD, RED, RESET),
        'a' => format!("{}{}a{}", BOLD, YELLOW, RESET),
        '$' => format!("{}{}${}", BOLD, GREEN, RESET),
        's' => format!("{}{}s{}", BOLD, CYAN, RESET),
        '.' => format!("{}{}.{}", DIM, "\x1b[37m", RESET),
        other => other.to_string(),
    }
}

struct Args {
    path: Option<String>,
    delay_ms: u64,
}

fn parse_args() -> Args {
    let mut path = None;
    let mut delay_ms = 120u64;
    let mut args = env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--delay" | "-d" => {
                if let Some(v) = args.next() {
                    delay_ms = v.parse().unwrap_or(delay_ms);
                }
            }
            "--help" | "-h" => {
                eprintln!("Usage: visualizer [file] [--delay MS]");
                std::process::exit(0);
            }
            other => {
                if path.is_none() {
                    path = Some(other.to_string());
                }
            }
        }
    }
    Args { path, delay_ms }
}

fn render_frame(header: &str, rows: &[String], out: &mut impl Write) -> io::Result<()> {
    write!(out, "{}", CLEAR)?;
    writeln!(out, "{}{}{}", BOLD, header, RESET)?;
    writeln!(out)?;
    for r in rows {
        let mut s = String::with_capacity(r.len() * 6);
        for c in r.chars() {
            s.push_str(&colorize(c));
        }
        writeln!(out, "  {}", s)?;
    }
    writeln!(out)?;
    writeln!(out, "{}Legend:{} {} P1 own  {} P1 last  {} P2 own  {} P2 last",
        BOLD, RESET, colorize('@'), colorize('a'), colorize('$'), colorize('s'))?;
    out.flush()
}

fn run<R: BufRead>(mut reader: R, delay: Duration) -> io::Result<()> {
    let stdout = io::stdout();
    let mut out = stdout.lock();

    let mut line = String::new();
    let mut frame: u64 = 0;

    loop {
        line.clear();
        let n = reader.read_line(&mut line)?;
        if n == 0 {
            break;
        }
        let trimmed = line.trim_end_matches(&['\n', '\r'][..]);
        if !trimmed.starts_with("Anfield ") {
            continue;
        }

        let header_text = trimmed.to_string();
        let dims = header_text
            .trim_start_matches("Anfield ")
            .trim_end_matches(':');
        let parts: Vec<&str> = dims.split_whitespace().collect();
        if parts.len() < 2 {
            continue;
        }
        let height: usize = match parts[1].parse() {
            Ok(v) => v,
            Err(_) => continue,
        };

        let mut peek = String::new();
        let n2 = reader.read_line(&mut peek)?;
        if n2 == 0 {
            break;
        }
        let peek_trim = peek.trim_end_matches(&['\n', '\r'][..]).trim_start();
        let column_header =
            !peek_trim.is_empty() && peek_trim.chars().all(|c| c.is_ascii_digit());

        let mut rows: Vec<String> = Vec::with_capacity(height);
        if !column_header {
            rows.push(strip_row_prefix(peek.trim_end_matches(&['\n', '\r'][..])).to_string());
        }
        while rows.len() < height {
            let mut row = String::new();
            let nr = reader.read_line(&mut row)?;
            if nr == 0 {
                break;
            }
            let r = row.trim_end_matches(&['\n', '\r'][..]);
            rows.push(strip_row_prefix(r).to_string());
        }

        frame += 1;
        let header = format!("Filler  -  frame #{}  -  {}", frame, header_text);
        render_frame(&header, &rows, &mut out)?;
        sleep(delay);
    }

    Ok(())
}

fn strip_row_prefix(line: &str) -> &str {
    let mut chars = line.chars();
    let a = chars.next();
    let b = chars.next();
    let c = chars.next();
    let d = chars.next();
    if let (Some(a), Some(b), Some(c), Some(d)) = (a, b, c, d) {
        if a.is_ascii_digit() && b.is_ascii_digit() && c.is_ascii_digit() && d == ' ' {
            return chars.as_str();
        }
    }
    line
}

fn main() -> io::Result<()> {
    let args = parse_args();
    let delay = Duration::from_millis(args.delay_ms);

    match args.path {
        Some(p) => {
            let f = File::open(p)?;
            run(BufReader::new(f), delay)
        }
        None => {
            let stdin = io::stdin();
            run(stdin.lock(), delay)
        }
    }
}
