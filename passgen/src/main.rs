mod passgen;

fn main() {
    let password = passgen::random_password_characters(passgen::CHARSET_UPPERCASE_HEX, 16);
    println!("The password is: {}", password);
}
