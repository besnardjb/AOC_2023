use std::{
    error::Error,
    fs::File,
    io::{BufReader, Read},
};

#[derive(Debug)]
struct Obs {
    r: i32,
    g: i32,
    b: i32,
}

#[derive(Debug)]
struct Game {
    id: i32,
    views: Vec<Obs>,
}

impl Game {
    fn new(id: i32) -> Game {
        Game {
            id,
            views: Vec::new(),
        }
    }

    fn possible(&self, r: i32, g: i32, b: i32) -> bool {
        for obs in self.views.iter() {
            if (obs.r > r) || (obs.g > g) || (obs.b > b) {
                println!(
                    "Impossible {} {} {} over {} {} {}",
                    r, g, b, obs.r, obs.g, obs.b
                );
                return false;
            }
        }

        true
    }

    fn min(&self) -> (i32, i32, i32) {
        let r: i32 = self.views.iter().map(|v| v.r).max().unwrap();
        let g: i32 = self.views.iter().map(|v| v.g).max().unwrap();
        let b: i32 = self.views.iter().map(|v| v.b).max().unwrap();

        (r, g, b)
    }

    fn power(&self) -> i32 {
        let m = self.min();
        m.0 * m.1 * m.2
    }

    fn push(&mut self, val: String) -> Result<(), Box<dyn Error>> {
        for obs in val.split(';') {
            let mut new = Obs { r: 0, g: 0, b: 0 };

            for vals in obs.split(',') {
                let data: Vec<String> = vals.trim().split(' ').map(|v| v.to_string()).collect();
                if data.len() != 2 {
                    println!("Error on {}", obs);
                }
                let cnt = data[0].parse::<i32>()?;

                match data[1].as_str() {
                    "red" => new.r += cnt,
                    "green" => new.g += cnt,
                    "blue" => new.b += cnt,
                    _ => println!("No such color {}", data[1]),
                }
            }

            println!("Add R {} G {} B {}", new.r, new.g, new.b);

            self.views.push(new);
        }

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./data.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let mut games: Vec<Game> = Vec::new();

    for line in contents.split('\n') {
        if !line.starts_with("Game") {
            println!("Skipping {}", line);
            continue;
        }

        let colon = line.find(':').unwrap();
        let id = String::from(&line[5..colon]).parse::<i32>()?;
        let rounds = String::from(&line[colon + 1..]);

        let mut g = Game::new(id);
        g.push(rounds)?;

        games.push(g);
    }

    let sumpossible: i32 = games
        .iter()
        .filter(|v| v.possible(12, 13, 14))
        .map(|v| v.id)
        .sum();

    let sumpower: i32 = games.iter().map(|v| v.power()).sum();

    println!("Sum of possible : {} sumpower : {}", sumpossible, sumpower);

    Ok(())
}
