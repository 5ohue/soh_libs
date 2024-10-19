use clap::Parser;
use soh_rng::prelude::*;

static LETTERS_LOWER: &str = "qwertyuiopasdfghjklzxcvbnm";
static LETTERS_UPPER: &str = "QWERTYUIOPASDFGHJKLZXCVBNM";
static DIGITS: &str = "12345678901234567890";
static SYMBOLS: &str = "(~`!@#$%^&*_-+={[]}\\:!?/)";

/// Generate simple random passwords.
/// You should probably not use them for anything serious.
#[derive(Parser)]
#[command(version, about)]
struct Args {
    /// Password length
    #[arg(short, long, default_value_t = 25)]
    length: usize,
    #[arg(short, long, default_value_t = 10)]
    /// Number of generated passwords
    num_of_passwords: usize,
    /// Random generator seed
    #[arg(short, long, default_value_t = 123456789)]
    seed: u64,

    /// Do not use lowercase letters
    #[arg(long, default_value_t = false)]
    no_letters_lower: bool,
    /// Do not use uppercase letters
    #[arg(long, default_value_t = false)]
    no_letters_upper: bool,
    /// Do not use digits
    #[arg(long, default_value_t = false)]
    no_digits: bool,
    /// Do not use symbols
    #[arg(long, default_value_t = false)]
    no_symbols: bool,
}

fn main() -> Result<(), ()> {
    let args = Args::parse();

    if args.no_letters_lower && args.no_letters_upper && args.no_digits && args.no_symbols {
        eprint!("Cannot generated password when all characters are disabled!");
        return Err(());
    }

    let alphabet = if args.no_letters_lower {
        "".chars()
    } else {
        LETTERS_LOWER.chars()
    }
    .chain(if args.no_letters_upper {
        "".chars()
    } else {
        LETTERS_UPPER.chars()
    })
    .chain(if args.no_digits {
        "".chars()
    } else {
        DIGITS.chars()
    })
    .chain(if args.no_symbols {
        "".chars()
    } else {
        SYMBOLS.chars()
    })
    .collect::<Vec<_>>();

    let mut rng = soh_rng::Xoshiro256SS::new(args.seed);

    for _ in 1..=args.num_of_passwords {
        let pass = (0..args.length)
            .map(|_| {
                let idx = rng.gen_to(alphabet.len());
                alphabet[idx]
            })
            .collect::<String>();

        println!("{pass}");
    }

    return Ok(());
}
