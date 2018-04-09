extern crate passgenr;
extern crate getopts;
use getopts::Options;

// TODO: refactor this so that arguments are parsed into a struct, so that the argument parsing can
// be unit-tested

#[derive(Clone, Copy)]
enum CommandLineCharset {
    Hex,
    Ascii,
    Alpha,
    Digit,
    Lower,
    Words,
}

const PASSWORD_LENGTH: usize = 64;
const PASSWORD_WORD_COUNT: usize = 10;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let program = args[0].clone();

    let opts = {
        let mut opts = Options::new();

        opts.optflag("x", "hex", &format!("{} hexadecimal characters", PASSWORD_LENGTH));
        opts.optflag("a", "ascii", &format!("{} non-whitespace printable ASCII characters", PASSWORD_LENGTH));
        opts.optflag("n", "alpha", &format!("{} alphanumeric characters", PASSWORD_LENGTH));
        opts.optflag("d", "digit", &format!("{} decimal digits", PASSWORD_LENGTH));
        opts.optflag("l", "lower", &format!("{} lowercase alphabetic characters", PASSWORD_LENGTH));
        opts.optflag("w", "words", &format!("{} random words from a list of {}", PASSWORD_WORD_COUNT, passgenr::charsets::WORDS.len()));
        opts.optflag("h", "help", "show this help menu");
        opts.optopt("p", "password-count", "number of passwords to generate", "N");

        opts
    };

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => {
            print_usage(&program, opts, Some(&f.to_string()));
            std::process::exit(1);
        }
    };

    if !matches.free.is_empty() {
        print_usage(&program, opts, Some("There are unnecessary command-line arguments."));
        std::process::exit(1);
    }

    if matches.opt_present("h") {
        print_usage(&program, opts, None);
        std::process::exit(0);
    }

    let password_count : u32 = match matches.opt_str("p") {
        Some(s) => {
            match s.parse() {
                Ok(n) => n,
                Err(_) => {
                    print_usage(&program, opts, Some("The password count is not a valid number."));
                    std::process::exit(1);
                }
            }
        },
        None => 1
    };

    let mut charset : Option<CommandLineCharset> = None;

    let translation_table = [
        ("x", CommandLineCharset::Hex),
        ("a", CommandLineCharset::Ascii),
        ("n", CommandLineCharset::Alpha),
        ("d", CommandLineCharset::Digit),
        ("l", CommandLineCharset::Lower),
        ("w", CommandLineCharset::Words)
    ];

    for option in translation_table.iter() {
        if matches.opt_present(option.0) {
            if let Some(_) = charset {
                print_usage(&program, opts, Some("Only one kind of password can be generated at a time."));
                std::process::exit(1);
            }
            charset = Some(option.1);
        }
    }

    for _ in 0..password_count {
        match charset {
            None => {
                print_usage(&program, opts, Some("Please specify the character set to generate the password from."));
                std::process::exit(1);
            },
            Some(CommandLineCharset::Hex) => {
                println!("{}", passgenr::random_password(passgenr::charsets::UPPERCASE_HEX, PASSWORD_LENGTH, "").unwrap());
            },
            Some(CommandLineCharset::Ascii) => {
                println!("{}", passgenr::random_password(passgenr::charsets::ASCII, PASSWORD_LENGTH, "").unwrap());
            },
            Some(CommandLineCharset::Alpha) => {
                println!("{}", passgenr::random_password(passgenr::charsets::ALPHANUMERIC, PASSWORD_LENGTH, "").unwrap());
            },
            Some(CommandLineCharset::Digit) => {
                println!("{}", passgenr::random_password(passgenr::charsets::DECIMAL_DIGIT, PASSWORD_LENGTH, "").unwrap());
            },
            Some(CommandLineCharset::Lower) => {
                println!("{}", passgenr::random_password(passgenr::charsets::LOWERCASE_ALPHABETIC, PASSWORD_LENGTH, "").unwrap());
            },
            Some(CommandLineCharset::Words) => {
                println!("{}", passgenr::random_password(passgenr::charsets::WORDS, PASSWORD_WORD_COUNT, ".").unwrap());
            }
        }
    }
}

fn print_usage(program: &str, opts: Options, error: Option<&str>) {
    if let Some(err_message) = error {
        println!("Error: {}", err_message);
    }
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}
