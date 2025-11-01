use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    let sol = match input.chars().next() {
        Some(first) => {
            let first_upper = first.to_uppercase().collect::<String>();
            let rest = &input[first.len_utf8()..];
            format!("{}{}", first_upper, rest)
        }
        None => String::new(),
    };

    println!("{sol}");
}
