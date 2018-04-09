extern crate rand;

pub mod charsets;

use self::rand::{OsRng, Rng};

/// Randomly generate a password made of `count` elements selected uniformly from `charset`,
/// separated by `separator`.
///
///     assert_eq!(
///         20,
///         passgenr::random_password(passgenr::charsets::ASCII, 20, "").unwrap().len()
///     );
///
/// The call will panic if `charset` contains fewer than two elements. The caller is responsible
/// for ensuring that the elements of `charset` are distinct (this is not verified internally).
///
/// Common character sets (e.g. ASCII, ALPHANUMERIC) are available in the `charsets` module.
pub fn random_password(charset: &[&str], count: usize, separator: &str) -> Result<String,std::io::Error> {
    Ok(random_password_elements(charset, count)?.join(separator))
}

fn random_password_elements<'a>(charset: &[&'a str], count: usize) -> Result<Vec<&'a str>,std::io::Error> {
    if charset.len() < 2 {
        panic!("The character set is too small (only 0 or 1 elements) to generate distinct passwords!");
    }

    if slice_contains_duplicates(charset) {
        panic!("The character set contained duplicate elements!");
    }

    let mut password_elts = Vec::<&str>::with_capacity(count);
    let mut rng = OsRng::new()?;

    for _ in 0..count {
        password_elts.push(rng.choose(charset).unwrap());
    }

    Ok(password_elts)
}

fn slice_contains_duplicates<T: Clone + Ord>(slice: &[T]) -> bool {
    let unique_elts : Vec<T> = {
        let mut elts = slice.to_vec();
        elts.sort();
        elts.dedup();
        elts
    };
    slice.len() != unique_elts.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    static ALL_INCLUDED_CHARSETS: [&[&str]; 7] = [
        charsets::UPPERCASE_HEX,
        charsets::LOWERCASE_HEX,
        charsets::ALPHANUMERIC,
        charsets::ASCII,
        charsets::DECIMAL_DIGIT,
        charsets::LOWERCASE_ALPHABETIC,
        charsets::WORDS,
    ];

    #[test]
    fn expected_charset_sizes() {
        assert_eq!(charsets::UPPERCASE_HEX.len(), 16);
        assert_eq!(charsets::LOWERCASE_HEX.len(), 16);
        assert_eq!(charsets::ALPHANUMERIC.len(), 26*2 + 10);
        assert_eq!(charsets::ASCII.len(), 94);
        assert_eq!(charsets::DECIMAL_DIGIT.len(), 10);
        assert_eq!(charsets::LOWERCASE_ALPHABETIC.len(), 26);
    }

    #[test]
    fn no_duplicates_in_charsets() {
        for charset in ALL_INCLUDED_CHARSETS.iter() {
            assert!(!slice_contains_duplicates(charset));
        }
    }

    #[test]
    fn passwords_are_correct_length() {
        for charset in ALL_INCLUDED_CHARSETS.iter() {
            for len in 0..50 {
                assert_eq!(random_password_elements(charset, len).unwrap().len(), len);
            }
        }

        for len in 0..50 {
            assert_eq!(random_password(charsets::ALPHANUMERIC, len, "").unwrap().len(), len);
        }

        // with separator
        assert_eq!(random_password(charsets::ALPHANUMERIC, 0, ".").unwrap().len(), 0);
        for len in 1..50 {
            assert_eq!(random_password(charsets::ALPHANUMERIC, len, ".").unwrap().len(), len + len - 1);
        }
    }

    #[test]
    fn no_chars_outside_requested_charset() {
        for charset in ALL_INCLUDED_CHARSETS.iter() {
            let password_elts = random_password_elements(charset, charset.len() * 5).unwrap();
            for elt in password_elts {
                assert!(charset.contains(&elt));
            }
        }
    }

    #[test]
    fn all_characters_seen_in_output_eventually() {
        for charset in ALL_INCLUDED_CHARSETS.iter() {
            let password_elts = random_password_elements(charset, charset.len() * 100).unwrap();
            for ch in charset.iter() {
                assert!(password_elts.contains(&ch));
            }
        }
    }

    #[test]
    fn custom_character_set() {
        let charset = vec!["0", "1"];
        match random_password(&charset, 2, "").unwrap().as_ref() {
            "00" => { },
            "01" => { },
            "10" => { },
            "11" => { },
            _ => panic!("Custom character set is broken")
        };
    }

    #[test]
    #[should_panic(expected = "character set is too small")]
    fn panics_on_empty_character_set() {
        let _ = random_password(&vec![], 2, "");
    }

    #[test]
    #[should_panic(expected = "character set is too small")]
    fn panics_on_size_one_character_set() {
        let _ = random_password(&vec!["a"], 2, "");
    }

    #[test]
    fn separator_works() {
        let charset = vec!["0", "1"];

        match random_password(&charset, 1, "###").unwrap().as_ref() {
            "0" => { },
            "1" => { },
            _ => panic!("Separator is broken for length-1 passwords")
        };

        match random_password(&charset, 2, "###").unwrap().as_ref() {
            "0###0" => { },
            "0###1" => { },
            "1###0" => { },
            "1###1" => { },
            _ => panic!("Separator is broken for length-2 passwords")
        };

        match random_password(&charset, 3, "###").unwrap().as_ref() {
            "0###0###0" => { },
            "0###0###1" => { },
            "0###1###0" => { },
            "0###1###1" => { },
            "1###0###0" => { },
            "1###0###1" => { },
            "1###1###0" => { },
            "1###1###1" => { },
            _ => panic!("Separator is broken for length-3 passwords")
        };
    }
}
