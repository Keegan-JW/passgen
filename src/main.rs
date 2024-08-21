use clap::Parser;
use copypasta::{ClipboardContext, ClipboardProvider};
use rand::seq::SliceRandom;
use std::process::exit;

/// Generate random passwords
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The length of the password
    #[arg(short, long, default_value_t = 20)]
    length: usize,

    #[arg(short = 'u', long)]
    include_uppercase: bool,

    #[arg(short = 'c', long)]
    include_lowercase: bool,

    #[arg(short = 'd', long)]
    include_digits: bool,

    #[arg(short = 's', long)]
    include_special: bool,

    #[arg(short = 'v', long)]
    copy_password: bool,
}

fn generate_password(args: &Args) -> String {
    let mut password_chars: Vec<char> = Vec::new();

    if args.include_uppercase {
        password_chars.extend("ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars());
    }
    if args.include_lowercase {
        password_chars.extend("abcdefghijklmnopqrstuvwxyz".chars());
    }
    if args.include_digits {
        password_chars.extend("0123456789".chars());
    }
    if args.include_special {
        password_chars.extend("!@#$%^&*()-_=+[{]}\\|;:'\",<.>/?".chars());
    }

    if password_chars.is_empty() {
        eprintln!("You need to have at least one character option");
        exit(1); // Exit program here to avoid carrying on with an empty string
    }

    let mut rng = rand::thread_rng();
    let password: String = (0..args.length)
        .map(|_| *password_chars.choose(&mut rng).unwrap())
        .collect();

    password
}

fn copy_password(password: &String) {
    let mut ctx = match ClipboardContext::new() {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Unable to get access to the clipboard.");
            return;
        }
    };

    if ctx.set_contents(password.to_owned()).is_err() {
        eprintln!("Unable to set the contents of the clipboard.");
        return;
    }
}

fn main() {
    let args = Args::parse();
    let password = generate_password(&args);

    if args.copy_password {
        copy_password(&password);
    }

    println!("{}", password);
}
