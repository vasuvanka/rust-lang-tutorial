use std::io;

fn main() {
    println!("Please enter your name >> ");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("failed to read input name");

    println!("hello, {}", name);
}
