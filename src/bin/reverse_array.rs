// Reverse an array of integers.

use std::io::{self};
use std::num::ParseIntError;

fn reverse_array(mut buffer: Vec<i32>) -> Result<String, ParseIntError> {
    let len = buffer.len();
    if len == 0 {
        return Ok(String::from(""));
    }
    for i in 0..len / 2 {
        let temp = buffer[len - 1 - i];
        buffer[len - 1 - i] = buffer[i];
        buffer[i] = temp;
    }
    let mut res = buffer.iter().map(|&b| b.to_string() + " ").collect::<String>();
    res.truncate((len * 2) - 1);
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(reverse_array(vec![]), Ok(String::from("")));
    }

    #[test]
    fn test_even() {
        let vec: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(reverse_array(vec), Ok(String::from("6 5 4 3 2 1")));
    }

    #[test]
    fn test_odd() {
        let vec: Vec<i32> = vec![1, 2, 3, 4, 5];
        assert_eq!(reverse_array(vec), Ok(String::from("5 4 3 2 1")));
    }
}


fn main() -> io::Result<()> {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer)?;
    let vec = buffer
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap()).collect();
    match reverse_array(vec) {
        Ok(reversed) => println!("{}", reversed),
        Err(e) => println!("Error: {}", e),
    };

    Ok(())
}