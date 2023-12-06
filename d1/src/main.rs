use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./data.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let mut total = 0;

    for line in contents.split('\n') {
        let replacee = vec![
            ("one", "1"),
            ("two", "2"),
            ("three", "3"),
            ("four", "4"),
            ("five", "5"),
            ("six", "6"),
            ("seven", "7"),
            ("eight", "8"),
            ("nine", "9"),
        ];

        let mut sline = line.to_string();

        for rep in replacee.iter() {
            sline = sline.replace(rep.0, format!("{}{}{}", rep.0, rep.1, rep.0).as_str());
            println!("{}", sline);
        }

        let values: Vec<i32> = sline
            .as_bytes()
            .iter()
            .filter(|v| (**v <= b'9') & (b'0' <= **v))
            .filter_map(|v| {
                if let Ok(s) = String::from_utf8(vec![*v]) {
                    if let Ok(v) = s.parse::<i32>() {
                        return Some(v);
                    }
                }
                None
            })
            .collect();

        if !values.is_empty() {
            println!("{:?}", values);
            let values: i32 = values[0] * 10 + values[values.len() - 1];
            println!("FL {}", values);

            total += values;
        }
    }

    println!("Total: {total}");

    Ok(())
}
