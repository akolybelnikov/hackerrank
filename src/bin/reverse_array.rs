// Reverse an array of integers.

use std::io::{self};

fn reverse_array(mut buffer: Vec<i32>) {
    let len = buffer.len();
    for i in 0..len / 2 {
        let temp = buffer[len - 1 - i];
        buffer[len - 1 - i] = buffer[i];
        buffer[i] = temp;
    }
    let joined: String = buffer.iter().map( |&b| b.to_string() + " ").collect();
    println!("{}", joined);
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer)?;
    let vec = buffer.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();

    reverse_array(vec);

    Ok(())
}