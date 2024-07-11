use rand::seq::SliceRandom;
use structopt::StructOpt;
use copypasta::{ClipboardContext, ClipboardProvider};

#[derive(Debug, StructOpt)]
#[structopt(name = "passgen", about = "Generate random passwords")]
struct Opt {
    #[structopt(short = "l", long, default_value = "20")]
    length: usize,

    #[structopt(short = "u", long)]
    include_uppercase: bool,

    #[structopt(short = "c", long)]
    include_lowercase: bool,

    #[structopt(short = "d", long)]
    include_digits: bool,

    #[structopt(short = "s", long)]
    include_special: bool,
    
    #[structopt(short = "v", long)]
    copy_password: bool,
}

fn generate_password(opt: &Opt) -> String {
    let mut password_chars: Vec<char> = Vec::new();

    if opt.include_uppercase {
        password_chars.extend("ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars());
    }
    if opt.include_lowercase {
        password_chars.extend("abcdefghijklmnopqrstuvwxyz".chars());
    }
    if opt.include_digits {
        password_chars.extend("0123456789".chars());
    }
    if opt.include_special {
        password_chars.extend("!@#$%^&*()-_=+[{]}\\|;:'\",<.>/?".chars());
    }

    if password_chars.is_empty() {
        println!("You need to have at least one character option");
    }

    let mut rng = rand::thread_rng();
    let password: String = (0..opt.length)
        .map(|_| *password_chars.choose(&mut rng).unwrap())
        .collect();

    password
}

fn copy_password(password: &String) {
    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_contents(password.to_owned()).unwrap();
    let _content = ctx.get_contents().unwrap();
}

fn main() {
    let opt = Opt::from_args();
    let password = generate_password(&opt);
    if opt.copy_password { copy_password(&password); }
    println!("{}", password);
}
