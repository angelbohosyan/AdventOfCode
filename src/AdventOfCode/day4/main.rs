use std::collections::HashSet;
use std::fs;
use std::io;

///22897

fn main() -> Result<(), io::Error> {
    let file_path = "input.txt";
    let result = fs::read_to_string(file_path)?;

    let mut sum = 0;
    for originalValue in result.lines() {
        let content: String = originalValue.replace("  "," ");
        let start: usize = content.find(":").unwrap();
        let sides: Vec<&str> = content[start+1..].split("|").collect();
        let mut winning: HashSet<&str> = HashSet::new();
        sides[0].split(" ").for_each(|f| {
            if f != "" {
                winning.insert(f);
            }
        });

        let mut result = 1;

        sides[1].split(" ").for_each(|f| {
            if winning.contains(&f) {
                result *= 2
            }
        });

        sum = sum + (result/2)
    }

    println!("Result is {}", sum);

    Ok(())
}