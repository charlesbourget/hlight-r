use regex::Regex;
use std::{env, io::BufRead, process::exit};

const ANSI_RED_BACKGROUND: &str = "\u{001B}[41m";
const ANSI_GREEN_BACKGROUND: &str = "\u{001B}[42m";
const ANSI_YELLOW_BACKGROUND: &str = "\u{001B}[43m";
const ANSI_BLUE_BACKGROUND: &str = "\u{001B}[44m";
const ANSI_PURPLE_BACKGROUND: &str = "\u{001B}[45m";
const ANSI_CYAN_BACKGROUND: &str = "\u{001B}[46m";
const ANSI_WHITE_BACKGROUND: &str = "\u{001B}[47m";
const ANSI_RESET: &str = "\u{001B}[0m";

const COLORS: [&str; 7] = [
    ANSI_RED_BACKGROUND,
    ANSI_GREEN_BACKGROUND,
    ANSI_YELLOW_BACKGROUND,
    ANSI_BLUE_BACKGROUND,
    ANSI_PURPLE_BACKGROUND,
    ANSI_CYAN_BACKGROUND,
    ANSI_WHITE_BACKGROUND,
];

struct Pattern {
    color: String,
    regex: Regex,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    validate_args(&args, COLORS.len());

    let patterns: Vec<Pattern> = build_pattern_vector(&args);

    for line in std::io::stdin().lock().lines() {
        let line = line.expect("Error reading line from stdin");

        println!("{}", color_line(line, &patterns));
    }
}

fn color_line(line: String, patterns: &Vec<Pattern>) -> String {
    for pattern in patterns {
        if pattern.regex.is_match(&line) {
            return format!("{}{}{}", &pattern.color, &line, ANSI_RESET);
        }
    }

    line
}

fn build_pattern_vector(args: &Vec<String>) -> Vec<Pattern> {
    args[1..]
        .iter()
        .zip(COLORS.iter())
        .map(|(regex, color)| Pattern {
            color: String::from(*color),
            regex: Regex::new(regex).expect("Error parsing regex"),
        })
        .collect()
}

fn validate_args(args: &Vec<String>, colors_len: usize) {
    if args.len() == 1 {
        println!("No arguments provided");
        usage();
    }

    if args.contains(&String::from("-h")) || args.contains(&String::from("--help")) {
        usage();
    }

    if args.len() > colors_len {
        println!("Too many arguments provided");
        usage();
    }
}

fn usage() {
    println!("Usage: {} [options] [patterns]", env!("CARGO_PKG_NAME"));
    println!("Options:");
    println!("  -h, --help\t\tShow this help message and exit");
    println!("Patterns:");
    println!("  Regular expressions to match against each line of input");
    exit(0);
}
