// Sort an array of strings so that uppercase strings follow the lowercase

use std::io::{self};

fn sort_strings(buffer: &str) -> String {
    let (mut l, mut u): (Vec<&str>, Vec<&str>) = buffer
        .split_whitespace()
        .partition(|&n| n.chars().next().unwrap().is_lowercase());
    l.sort_unstable();
    u.sort_unstable();
    l.append(&mut u);
    l.join(" ")
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer)?;
    println!("{:?}", sort_strings(&buffer));
    Ok(())
}