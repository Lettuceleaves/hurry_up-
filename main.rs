use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    print!("Enter the size of map, (and good luck): ");

    loop {
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read character");

        print!("{}", input);

        if input.chars().next() == Some('q') && input.len() == 1 {
            break;
        }
    }
}