
extern crate rand;

// TODO: make this side-channel safe, probably with the timing-sheild crate,
// once it becomes compilable with stable rust.

// FIXME: put the extern crate line in the proper place
use self::rand::{OsRng, Rng};

pub const CHARSET_UPPERCASE_HEX: &[char] = &[
    '0','1','2','3','4','5','6','7','8','9','A','B','C','D','E','F'
];
pub const CHARSET_LOWERCASE_HEX: &[char] = &[
    '0','1','2','3','4','5','6','7','8','9','a','b','c','d','e','f'
];
pub const CHARSET_ALPHANUMERIC: &[char] = &[
    'a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z',
    'A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z',
    '0','1','2','3','4','5','6','7','8','9'
];
pub const CHARSET_ASCII: &[char] = &[
    '!','"','#','$','%','&','\'','(',')','*','+',',','-','.','/',
    '0','1','2','3','4','5','6','7','8','9',
    ':',';','<','=','>','?','@',
    'A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z',
    '[','\\',']','^','_','`',
    'a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z',
    '{','|','}','~'
];
pub const CHARSET_DECIMAL_DIGIT: &[char] = &[
    '0','1','2','3','4','5','6','7','8','9'
];
pub const CHARSET_LOWERCASE_ALPHABETIC: &[char] = &[
    'a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'
];

// TODO: make charset type which enforces non-duplicate characters

pub fn random_password_characters(charset: &[char], length: usize) -> Vec<char> {
    let mut password = Vec::<char>::with_capacity(length);

    let mut rng = match OsRng::new() {
        Ok(rng) => { rng },
        Err(_) => { panic!("Random number generator failure."); }
    };

    for _ in 0..length {
        password.push(sample_random_char(&mut rng, &charset));
    }

    password
}

// pub fn random_password_words(num_words: usize) -> Vec<&'static str> {
// 
// }

fn sample_random_char(rng: &mut OsRng, charset: &[char]) -> char {
    let mask = charset.len().next_power_of_two() - 1;
    let mut random_index = rng.next_u32() as usize & mask;
    while random_index >= charset.len() {
        random_index = rng.next_u32() as usize & mask;
    }

    charset[random_index]
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALL_INCLUDED_CHARSETS: [&[char]; 6] = [
            &CHARSET_UPPERCASE_HEX,
            &CHARSET_LOWERCASE_HEX,
            &CHARSET_ALPHANUMERIC,
            &CHARSET_ASCII,
            &CHARSET_DECIMAL_DIGIT,
            &CHARSET_LOWERCASE_ALPHABETIC,
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
        for charset in ALL_INCLUDED_CHARSETS.iter() {
            let unique_chars : Vec<char> = {
                let mut chars = charset.to_vec();
                chars.sort();
                chars.dedup();
                chars
            };
            assert_eq!(charset.len(), unique_chars.len());
        }
    }

    #[test]
    fn passwords_are_correct_length() {
        for charset in ALL_INCLUDED_CHARSETS.iter() {
            for len in 0..50 {
                assert_eq!(random_password_characters(charset, len).len(), len);
            }
        }
    }

    #[test]
    fn no_chars_outside_requested_charset() {
        for charset in ALL_INCLUDED_CHARSETS.iter() {
            let password = random_password_characters(charset, 1000);
            for ch in password {
                assert!(charset.contains(&ch));
            }
        }
    }

    #[test]
    fn all_characters_seen_in_output_eventually() {
        for charset in ALL_INCLUDED_CHARSETS.iter() {
            let password = random_password_characters(charset, charset.len() * 100);
            for ch in charset.iter() {
                assert!(password.contains(&ch));
            }
        }
    }

    // TODO: frequency analysis (test fail with at most 1/100,000 probability)
    // #[test]
    // fn frequency_analysis() {

    // }
}
