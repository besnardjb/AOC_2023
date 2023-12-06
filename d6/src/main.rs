use std::fs::File;
use std::io::{BufReader, Read};

struct Run {
    record: i64,
    dist: i64,
}

impl Run {
    fn new(record: i64, dist: i64) -> Run {
        Run { record, dist }
    }

    fn numpos(&self) -> i64 {
        let mut ret = 0;

        for h in 1..self.record {
            if h * (self.record - h) > self.dist {
                //println!("Hold {} works", h);
                ret += 1;
            }
        }

        ret
    }
}

fn parse_num_list(data: &str) -> Vec<i64> {
    let mut ret: Vec<i64> = Vec::new();

    for v in data.split(' ') {
        if let Ok(v) = v.trim().parse::<i64>() {
            ret.push(v);
        }
    }

    ret
}

fn load_data(data: &str, merge_spaces: bool) -> (Vec<i64>, Vec<i64>) {
    let lines: Vec<&str> = data.split('\n').collect();

    assert!(lines.len() == 2);

    // Part 1
    let times = lines[0]["Time:".len()..].to_string();
    let distances = lines[1]["Distance:".len()..].to_string();
    let times = times.trim().to_string();
    let distances = distances.trim().to_string();

    let (times, distances) = if merge_spaces {
        (times.replace(' ', ""), distances.replace(' ', ""))
    } else {
        (times, distances)
    };

    let times = parse_num_list(times.as_str());
    let distances = parse_num_list(distances.as_str());

    assert!(times.len() == distances.len());

    (times, distances)
}

fn main() {
    let f = File::open("data.txt").unwrap();
    let mut r = BufReader::new(f);

    let mut data = String::new();
    r.read_to_string(&mut data).unwrap();

    // Part 1
    let mut runs: Vec<Run> = Vec::new();

    let (times, distances) = load_data(&data, false);

    for i in 0..times.len() {
        runs.push(Run::new(times[i], distances[i]));
    }

    let prod: i64 = runs.iter().map(|v| v.numpos()).product();

    println!("PART 1 : {}", prod);

    // Part 2
    let (times, distances) = load_data(&data, true);

    runs.clear();

    for i in 0..times.len() {
        runs.push(Run::new(times[i], distances[i]));
    }

    let prod: i64 = runs.iter().map(|v| v.numpos()).product();

    println!("PART 2 : {}", prod);
}
