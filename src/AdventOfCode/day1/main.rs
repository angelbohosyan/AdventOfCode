use std::fs;
use std::io;

///54877

fn main() -> Result<(), io::Error> {
    let file_path = "input.txt";
    let result = fs::read_to_string(file_path)?;

    let mut sum = 0;
    for content in result.lines() {
        for c in content.chars() {
            if c >= '0' && c <= '9' {
                if let Some(result) = c.to_digit(10) {
                    sum += result * 10;
                    break
                }
            }
        }

        for c in content.chars().rev() {
            if c >= '0' && c <= '9' {
                if let Some(result) = c.to_digit(10) {
                    sum += result;
                    break
                }
            }
        }
    }

    println!("Result is {}", sum);

    Ok(())
}