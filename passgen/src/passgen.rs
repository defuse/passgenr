
extern crate rand;

// FIXME: put the extern crate line in the proper place
use self::rand::{OsRng, Rng};

pub const CHARSET_UPPERCASE_HEX: &'static str = "0123456789ABCDEF";
pub const CHARSET_LOWERCASE_HEX: &'static str = "0123456789abcdef";
pub const CHARSET_ALPHANUMERIC: &'static str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
pub const CHARSET_ASCII: &'static str = "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";
pub const CHARSET_DECIMAL_DIGIT: &'static str = "0123456789";
pub const CHARSET_LOWERCASE_ALPHABETIC: &'static str = "abcdefghijklmnopqrstuvwxyz";

pub fn random_password_characters(charset: &str, length: usize) -> String {
    let mut password : String = String::new();

    let charset_vec = charset.chars().collect();

    let mut rng = match OsRng::new() {
        Ok(rng) => { rng },
        Err(_) => { panic!("Random number generator failure."); }
    };

    for _ in 0..length {
        password.push(sample_random_char(&mut rng, &charset_vec));
    }

    password
}

// pub fn random_password_words(num_words: usize) -> Vec<&'static str> {
// 
// }

fn sample_random_char(rng: &mut OsRng, charset: &Vec<char>) -> char {
    let mask = charset.len().next_power_of_two() - 1;
    let mut random_index = rng.next_u32() as usize & mask;
    while random_index >= charset.len() {
        random_index = rng.next_u32() as usize & mask;
    }

    // TODO: make this side-channel safe
    charset[random_index]
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALL_INCLUDED_CHARSETS: [&str; 6] = [
            CHARSET_UPPERCASE_HEX,
            CHARSET_LOWERCASE_HEX,
            CHARSET_ALPHANUMERIC,
            CHARSET_ASCII,
            CHARSET_DECIMAL_DIGIT,
            CHARSET_LOWERCASE_ALPHABETIC,
    ];

    #[test]
    fn expected_charset_sizes() {
        assert_eq!(CHARSET_UPPERCASE_HEX.len(), 16);
        assert_eq!(CHARSET_LOWERCASE_HEX.len(), 16);
        assert_eq!(CHARSET_ALPHANUMERIC.len(), 26*2 + 10);
        assert_eq!(CHARSET_ASCII.len(), 94);
        assert_eq!(CHARSET_DECIMAL_DIGIT.len(), 10);
        assert_eq!(CHARSET_LOWERCASE_ALPHABETIC.len(), 26);
    }

    #[test]
    fn no_duplicates_in_charsets() {
        for charset in &ALL_INCLUDED_CHARSETS {
            let unique_chars : Vec<char> = {
                let mut chars : Vec<char> = charset.chars().collect();
                chars.sort();
                chars.dedup();
                chars
            };
            assert_eq!(charset.len(), unique_chars.len());
        }
    }

    #[test]
    fn passwords_are_correct_length() {
        for charset in &ALL_INCLUDED_CHARSETS {
            for len in 0..50 {
                assert_eq!(random_password_characters(charset, len).len(), len);
            }
        }
    }

    #[test]
    fn no_chars_outside_requested_charset() {
        for charset in &ALL_INCLUDED_CHARSETS {
            let charset_vec : Vec<char> = charset.chars().collect();
            let password = random_password_characters(charset, 1000);
            for ch in password.chars() {
                assert!(charset_vec.contains(&ch));
            }
        }
    }

    #[test]
    fn all_characters_seen_in_output_eventually() {
        for charset in &ALL_INCLUDED_CHARSETS {
            let password = random_password_characters(charset, charset.len() * 100);
            let password_chars : Vec<char> = password.chars().collect();
            for ch in charset.chars() {
                assert!(password_chars.contains(&ch));
            }
        }
    }

    // TODO: frequency analysis (test fail with at most 1/100,000 probability)
    // #[test]
    // fn frequency_analysis() {

    // }
}
