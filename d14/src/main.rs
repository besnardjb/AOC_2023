use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{BufReader, Read};

use array2d::Array2D;

#[derive(Debug)]
struct Map {
    m: Array2D<char>,
}

impl Map {
    fn new(data: &str) -> Map {
        let mut lines: Vec<Vec<char>> = Vec::new();
        for l in data.split('\n') {
            let v: Vec<char> = l.as_bytes().iter().map(|v| *v as char).collect();
            lines.push(v);
        }

        Map {
            m: Array2D::from_rows(&lines).unwrap(),
        }
    }

    fn print(&self) {
        println!("========");
        for x in 0..self.m.row_len() {
            for y in 0..self.m.column_len() {
                let v = self.m.get(x, y).unwrap();
                print!("{}", v);
            }
            println!();
        }
    }

    fn aply_move(&mut self, pos: (usize, usize), dir: (i64, i64)) -> bool {
        let mut tx: i64 = pos.0 as i64;
        let mut ty: i64 = pos.1 as i64;

        let mut did_move = false;

        loop {
            let origx = tx as usize;
            let origy = ty as usize;
            tx += dir.0;
            ty += dir.1;

            let mut can_move = false;

            if let Some(v) = self.m.get(tx as usize, ty as usize) {
                if *v == '.' {
                    can_move = true;
                } else {
                    break;
                }
            } else {
                break;
            }

            if can_move {
                self.m.set(origx, origy, '.').unwrap();
                self.m.set(tx as usize, ty as usize, 'O').unwrap();
                did_move = true;
            }
        }

        did_move
    }

    fn move_blocks(&mut self, dir: (i64, i64)) {
        loop {
            let mut did_move = false;

            for x in 0..self.m.row_len() {
                for y in 0..self.m.column_len() {
                    let v = self.m.get(x, y).unwrap();
                    if *v == 'O' {
                        if self.aply_move((x, y), dir) {
                            did_move = true;
                        }
                    }
                }
            }

            if !did_move {
                break;
            }
        }
    }

    fn hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.m.hash(&mut hasher);
        hasher.finish()
    }

    fn period(&mut self) -> (u64, u64, u64) {
        let mut ret: u64 = 0;

        let mut known_layouts: HashMap<u64, u64> = HashMap::new();

        loop {
            ret += 1;
            self.cycle();
            let new = self.hash();

            if let Some(v) = known_layouts.get(&new) {
                println!("{} is resonating with {}", ret, v);
                return (ret, *v, ret - *v);
            } else {
                known_layouts.insert(new, ret);
            }
        }
    }

    fn cycle(&mut self) {
        self.move_blocks((-1, 0));
        self.move_blocks((0, -1));
        self.move_blocks((1, 0));
        self.move_blocks((0, 1));
    }

    fn score(&self) -> i64 {
        let mut ret: i64 = 0;
        for y in 0..self.m.column_len() {
            for x in 0..self.m.row_len() {
                let weight: i64 = (self.m.row_len() - x) as i64;
                let v = self.m.get(x, y).unwrap();
                if *v == 'O' {
                    ret += weight;
                }
            }
        }

        ret
    }
}

fn main() {
    let f = File::open("data.txt").unwrap();

    let mut r = BufReader::new(f);

    let mut data: String = String::new();

    r.read_to_string(&mut data).unwrap();

    let mut part1 = Map::new(&data);
    part1.move_blocks((-1, 0));
    println!("Part 1 Score is {}", part1.score());

    let mut part2_per = Map::new(&data);

    let period = part2_per.period();
    println!("Period is {:?}", period);

    let mut part2 = Map::new(&data);

    for _ in 0..(1000000000 - period.0) % period.2 + period.1 {
        part2.cycle();
    }

    println!("Part 2 Score is {}", part2.score());
}
