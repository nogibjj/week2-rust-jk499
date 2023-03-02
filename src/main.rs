use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter a line of text:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let reversed = reverse_string(input.trim());

    println!("Reversed text: {}", reversed);
}

fn reverse_string(s: &str) -> String {
    if s.is_empty() {
        String::new()
    } else {
        let (first, rest) = s.split_at(s.len() - 1);
        format!("{}{}", rest, reverse_string(first))
    }
}
