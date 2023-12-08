use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};

#[derive(Clone)]
struct PathWalker {
    path: Vec<u8>,
    current_off: usize,
}

impl PathWalker {
    fn new(path: &str) -> PathWalker {
        PathWalker {
            path: path.trim().as_bytes().to_vec(),
            current_off: 0,
        }
    }
}

impl Iterator for PathWalker {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = Some(*self.path.get(self.current_off).unwrap());
        self.current_off = (self.current_off + 1) % self.path.len();
        ret
    }
}

struct Map {
    edges: HashMap<String, (String, String)>,
}

impl Map {
    fn new(data: &str) -> Map {
        let mut edges: HashMap<String, (String, String)> = HashMap::new();

        for l in data.split('\n') {
            let sp: Vec<&str> = l.split('=').collect();
            assert!(sp.len() == 2);

            let name = sp[0].trim().to_string();

            let clean_str = sp[1].replace(['(', ')'], "");
            let choices: Vec<&str> = clean_str.split(',').collect();
            assert!(choices.len() == 2);

            let left = choices[0].trim().to_string();
            let right = choices[1].trim().to_string();

            edges.insert(name, (left, right));
        }

        println!("{:?}", edges);

        Map { edges }
    }

    fn goto(&self, dest: &str) -> Option<&(String, String)> {
        self.edges.get(dest)
    }

    fn next(&self, from: &str, choice: &u8) -> String {
        let cur = self.edges.get(from).unwrap();

        let next = match choice {
            b'L' => cur.0.as_str(),
            b'R' => cur.1.as_str(),
            _ => unreachable!("No such way {}", choice),
        };

        next.to_string()
    }

    fn start_nodes(&self) -> Vec<String> {
        self.edges
            .keys()
            .filter(|v| v.ends_with('A'))
            .cloned()
            .collect()
    }
}

fn main() {
    let f = File::open("data.txt").unwrap();
    let mut r = BufReader::new(f);

    let mut data = String::new();

    r.read_to_string(&mut data).unwrap();

    let data_split: Vec<&str> = data.split("\n\n").collect();
    assert!(data_split.len() == 2);

    let path = PathWalker::new(data_split[0]);
    let map = Map::new(data_split[1]);

    let mut cur = map.goto("AAA").unwrap();

    let mut cnt = 0;

    for v in path.into_iter() {
        let next = match v {
            b'L' => cur.0.as_str(),
            b'R' => cur.1.as_str(),
            _ => unreachable!("No such way {}", v),
        };

        let candi = map.goto(next);
        cnt += 1;

        if next == "ZZZ" {
            break;
        }

        if let Some(candi) = candi {
            cur = candi;
        } else {
            unreachable!("We are at a dead end ({}, {})", cur.0, cur.1);
        }
    }

    println!("Part 1 iter values is {}", cnt);

    // Part 2 Get Loop Length USING LCM
    let currents = map.start_nodes();
    let mut path = PathWalker::new(data_split[0]);

    let mut iter_vals: Vec<u64> = Vec::new();

    for c in currents.iter() {
        let mut cur = map.goto(c).unwrap();

        let mut cnt = 0;

        for v in path.clone().into_iter() {
            let next = match v {
                b'L' => cur.0.as_str(),
                b'R' => cur.1.as_str(),
                _ => unreachable!("No such way {}", v),
            };

            let candi = map.goto(next);
            cnt += 1;

            if next.ends_with('Z') {
                break;
            }

            if let Some(candi) = candi {
                cur = candi;
            } else {
                unreachable!("We are at a dead end ({}, {})", cur.0, cur.1);
            }
        }

        iter_vals.push(cnt);
        println!("{} iter values is {}", c, cnt);
    }

    let lcm = iter_vals.iter().fold(1, |a, b| num::integer::lcm(a, *b));

    println!("LCM {}", lcm);

    // Part 2 (Brute FOOORRRCE)

    let mut path = PathWalker::new(data_split[0]);

    let currents = map.start_nodes();

    let mut currents: Vec<String> = currents.iter().map(|v| v.to_string()).collect();

    let mut cnt: u64 = 0;
    let mut last_cnt = 0;

    let bar = ProgressBar::new(lcm);
    bar.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise} / {eta_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
        )
        .unwrap()
        .progress_chars("##-"),
    );

    loop {
        let choice = path.next().unwrap();
        currents = currents.iter().map(|v| map.next(v, &choice)).collect();

        let count_with_z = currents.iter().filter(|v| v.ends_with('Z')).count();

        cnt += 1;
        bar.inc(1);

        if count_with_z > 0 && (cnt - last_cnt) > 100000 {
            last_cnt = cnt;
        }

        if count_with_z == currents.len() {
            break;
        }
    }

    println!("Part 2 iter values is {}", cnt);
}
