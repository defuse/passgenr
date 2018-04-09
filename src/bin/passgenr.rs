extern crate passgenr;
extern crate getopts;
use getopts::Options;

#[derive(Clone, Copy, PartialEq, Debug)]
enum CommandLineCharset {
    Hex,
    Ascii,
    Alpha,
    Digit,
    Lower,
    Words,
}

#[derive(PartialEq, Debug)]
enum OptParseResult {
    Help,
    Generate(CommandLineCharset, u32),
    Err(String)
}

const PASSWORD_LENGTH: usize = 64;
const PASSWORD_WORD_COUNT: usize = 10;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let program = args[0].clone();

    let opts = prepare_opts();

    let (charset, password_count) = match parse_args(&opts, &args[1..]) {
        OptParseResult::Generate(c, n) => (c, n),
        OptParseResult::Help => {
            print_usage(&program, opts, None);
            std::process::exit(0);
        }
        OptParseResult::Err(err_msg) => {
            print_usage(&program, opts, Some(&err_msg));
            std::process::exit(1);
        }
    };

    for _ in 0..password_count {
        match charset {
            // we'll panic on CSPRNG failure
            CommandLineCharset::Hex =>{
                println!("{}", passgenr::random_password(passgenr::charsets::UPPERCASE_HEX, PASSWORD_LENGTH, "").unwrap());
            },
            CommandLineCharset::Ascii => {
                println!("{}", passgenr::random_password(passgenr::charsets::ASCII, PASSWORD_LENGTH, "").unwrap());
            },
            CommandLineCharset::Alpha => {
                println!("{}", passgenr::random_password(passgenr::charsets::ALPHANUMERIC, PASSWORD_LENGTH, "").unwrap());
            },
            CommandLineCharset::Digit => {
                println!("{}", passgenr::random_password(passgenr::charsets::DECIMAL_DIGIT, PASSWORD_LENGTH, "").unwrap());
            },
            CommandLineCharset::Lower => {
                println!("{}", passgenr::random_password(passgenr::charsets::LOWERCASE_ALPHABETIC, PASSWORD_LENGTH, "").unwrap());
            },
            CommandLineCharset::Words => {
                println!("{}", passgenr::random_password(passgenr::charsets::WORDS, PASSWORD_WORD_COUNT, ".").unwrap());
            }
        }
    }
}

fn prepare_opts() -> Options {
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
}

fn parse_args(opts: &Options, args: &[String]) -> OptParseResult {
    let matches = match opts.parse(args) {
        Ok(m) => { m }
        Err(f) => {
            return OptParseResult::Err(f.to_string());
        }
    };

    if !matches.free.is_empty() {
        return OptParseResult::Err("There are unnecessary command-line arguments.".to_owned());
    }

    if matches.opt_present("h") {
        return OptParseResult::Help;
    }

    let password_count : u32 = match matches.opt_str("p") {
        Some(s) => {
            match s.parse() {
                Ok(n) => n,
                Err(_) => {
                    return OptParseResult::Err("The password count is not a valid number.".to_owned());
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
                return OptParseResult::Err("Only one kind of password can be generated at a time.".to_owned());
            }
            charset = Some(option.1);
        }
    }

    if charset.is_none() {
        return OptParseResult::Err("Please specify which character set to generate the password from.".to_owned());
    }

    OptParseResult::Generate(charset.unwrap(), password_count)
}

fn print_usage(program: &str, opts: Options, error: Option<&str>) {
    if let Some(err_message) = error {
        println!("Error: {}", err_message);
    }
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_help() {
        let opts = prepare_opts();
        assert_eq!(OptParseResult::Help, parse_args(&opts, &["-h".to_owned()]));
    }

    #[test]
    fn test_charset_flags() {
        let translation_table = [
            ("x", CommandLineCharset::Hex),
            ("a", CommandLineCharset::Ascii),
            ("n", CommandLineCharset::Alpha),
            ("d", CommandLineCharset::Digit),
            ("l", CommandLineCharset::Lower),
            ("w", CommandLineCharset::Words)
        ];
        for option in translation_table.iter() {
            let opts = prepare_opts();
            assert_eq!(
                OptParseResult::Generate(option.1, 1),
                parse_args(&opts, &[format!("-{}", option.0)])
            );
        }
    }

    #[test]
    fn test_long_charset_flags() {
        let translation_table = [
            ("hex", CommandLineCharset::Hex),
            ("ascii", CommandLineCharset::Ascii),
            ("alpha", CommandLineCharset::Alpha),
            ("digit", CommandLineCharset::Digit),
            ("lower", CommandLineCharset::Lower),
            ("words", CommandLineCharset::Words)
        ];
        for option in translation_table.iter() {
            let opts = prepare_opts();
            assert_eq!(
                OptParseResult::Generate(option.1, 1),
                parse_args(&opts, &[format!("--{}", option.0)])
            );
        }
    }

    #[test]
    fn test_password_count() {
        let opts = prepare_opts();
        assert_eq!(
            OptParseResult::Generate(CommandLineCharset::Hex, 5),
            parse_args(&opts, &["-x".to_owned(), "-p".to_owned(), "5".to_owned()])
        );
    }

    #[test]
    fn test_extra_arguments() {
        let opts = prepare_opts();
        if let OptParseResult::Err(e) = parse_args(&opts, &["-h".to_owned(), "foo".to_owned()]) {
            assert!(e.contains("unnecessary command-line arguments"));
        } else {
            panic!("Argument parsing doesn't detect duplicate arguments.");
        }
    }

    #[test]
    fn test_unknown_argument() {
        let opts = prepare_opts();
        if let OptParseResult::Err(e) = parse_args(&opts, &["-z".to_owned()]) {
            assert!(e.contains("Unrecognized option"));
        } else {
            panic!("Argument parsing doesn't detect unknown options.");
        }
    }

    #[test]
    fn test_multiple_charset_flags() {
        let opts = prepare_opts();
        if let OptParseResult::Err(e) = parse_args(&opts, &["-x".to_owned(), "-a".to_owned()]) {
            assert!(e.contains("Only one kind"));
        } else {
            panic!("Argument parsing doesn't fail on multiple charset flags.");
        }
    }

    #[test]
    fn test_duplicate_charset_flags() {
        let opts = prepare_opts();
        if let OptParseResult::Err(e) = parse_args(&opts, &["-x".to_owned(), "-x".to_owned()]) {
            assert!(e.contains("given more than once"));
        } else {
            panic!("Argument parsing doesn't fail on duplicate charset flags.");
        }
    }

    #[test]
    fn test_invalid_password_count() {
        let opts = prepare_opts();
        if let OptParseResult::Err(e) = parse_args(&opts, &["-x".to_owned(), "-p".to_owned(), "foo".to_owned()]) {
            assert!(e.contains("not a valid number"));
        } else {
            panic!("Argument parsing doesn't fail on duplicate charset flags.");
        }
    }

    #[test]
    fn test_no_arguments() {
        let opts = prepare_opts();
        if let OptParseResult::Err(e) = parse_args(&opts, &[]) {
            assert!(e.contains("specify which character set"));
        } else {
            panic!("Argument parsing doesn't fail on an empty argument list.");
        }
    }
}
