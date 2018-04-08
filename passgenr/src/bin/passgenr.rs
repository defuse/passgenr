mod passgen;

fn main() {
    let x: &str = "fooooo";
    let password = passgen::random_password_characters(&x, 16);
   // println!("The password is: {}", password);
}
