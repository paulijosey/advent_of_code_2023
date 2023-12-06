use std::fs;

struct Game {
    name: String,
    id: i32,
    draws: Vec<Draw>,
    valid: bool,
}

struct Draw {
    red: i32,
    green: i32,
    blue: i32,
}

fn main() {
    // --snip--
    let file_path = "data/input.txt";
    println!("In file {}", file_path);

    let max_draw = Draw {
        red: 12,
        green: 13,
        blue: 14,
    };

    // get data and save it to a vector
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines: Vec<String> = contents.lines().map(|line| line.to_string()).collect();
    let mut games: Vec<Game> = lines
        .iter()
        .map(|line| {
            let parts: Vec<&str> = line.split(":").collect();
            let games = parts[0];
            let draws: Vec<&str> = parts[1].split(";").collect();
            Game {
                name: games.to_string(),
                draws: str_to_draws(draws.iter().map(|&draw| draw.to_string()).collect()),
                valid: true,
                id: str_to_id(games.to_string()),
            }
        })
        .collect();

    // parse each line
    for game in &mut games {
        for draw in &game.draws {
            if draw.red > max_draw.red || draw.green > max_draw.green || draw.blue > max_draw.blue {
                game.valid = false;
                break;
            }
        }
    }

    let mut result = 0;
    for game in &games {
        if game.valid {
            result += game.id;
        }
    }

    println!("Result 01: {}", result);

    result = 0;
    // Part 2
    // find min number of balls to make a valid game
    for game in &mut games {
        let mut min_draw = Draw {
            red: 0,
            green: 0,
            blue: 0,
        };
        for draw in &game.draws {
            if draw.red > min_draw.red {
                min_draw.red = draw.red;
            }
            if draw.green > min_draw.green {
                min_draw.green = draw.green;
            }
            if draw.blue > min_draw.blue {
                min_draw.blue = draw.blue;
            }
        }
        result += min_draw.red * min_draw.green * min_draw.blue;
    }
    println!("Result 02: {}", result);
}

fn str_to_draws(draws: Vec<String>) -> Vec<Draw> {
    let mut result: Vec<Draw> = Vec::new();
    for draw in draws {
        let parts: Vec<&str> = draw.split(",").collect();
        let mut red: i32 = 0;
        let mut green: i32 = 0;
        let mut blue: i32 = 0;
        for part in &parts {
            if part.contains("red") {
                red = part
                    .replace("red", "")
                    .trim()
                    .parse()
                    .expect("Unable to parse string to integer");
            } else if part.contains("green") {
                green = part
                    .replace("green", "")
                    .trim()
                    .parse()
                    .expect("Unable to parse string to integer");
            } else if part.contains("blue") {
                blue = part
                    .replace("blue", "")
                    .trim()
                    .parse()
                    .expect("Unable to parse string to integer");
            }
        }
        result.push(Draw {
            red: red,
            green: green,
            blue: blue,
        });
    }
    result
}

fn str_to_id(name: String) -> i32 {
    let id: i32 = name
        .replace("Game", "")
        .trim()
        .parse()
        .expect("Unable to parse string to integer");
    id
}
