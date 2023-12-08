use std::fs;
use std::io;

///2176

fn main() -> Result<(), io::Error> {
    let file_path = "input.txt";
    let result = fs::read_to_string(file_path)?;

    let mut sum = 0;
    for content in result.lines() {
        assert!(&content[..5]=="Game ");

        let mut number = String::new();
        let mut start_game=0;
        for (index, symbol) in content[5..].char_indices() {
            if symbol == ':' {
                start_game = 5+index;
                break
            } else {
                number.push(symbol)
            }
        }

        let game_id: i32 = number.parse().unwrap();
        let rounds: Vec<&str> = content[start_game+1..].split(';').collect();

        if evaluate_rounds(rounds) {
            sum += game_id
        }
    }

    println!("Result is {}", sum);
    Ok(())
}

fn evaluate_rounds(rounds: Vec<&str>) -> bool {
    for round in rounds {
        let grabs: Vec<&str> = round.split(",").collect();
        for grab in grabs {
            let grab_values: Vec<&str> = grab.split(" ").collect();
            let value: i32 = grab_values[1].parse().unwrap();
            if grab_values[2] == "blue" && value > 14 {
                return false
            } else if grab_values[2] == "green" && value > 13 {
                return false
            } else if grab_values[2] == "red" && value > 12 {
                return false
            }
        }
    }

    return true
}