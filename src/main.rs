use std::env;
use std::io::BufRead;
use regex::Regex;

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
    color: &'static str,
    regex: Regex,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    validate_args(&args, COLORS.len());

    let patterns: Vec<Pattern> = args[1..]
        .iter()
        .zip(COLORS.iter())
        .map(|(regex, color)| Pattern {
            color: color,
            regex: Regex::new(regex).unwrap(),
        })
        .collect();

    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        let mut output = String::new();

        for pattern in &patterns {
            if pattern.regex.is_match(&line) {
                output.push_str(pattern.color);
                output.push_str(&line);
                output.push_str(ANSI_RESET);
                break;
            }
        }

        if output.is_empty() {
            output.push_str(&line);
        }
        
        println!("{}", output);
    }
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
    println!("Usage: {} [options]", env!("CARGO_PKG_NAME"));
    println!("Options:");
    println!("  -h, --help\t\tShow this help message");
    std::process::exit(0);
}