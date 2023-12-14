use std::collections::hash_map::DefaultHasher;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{BufReader, Read};

#[derive(Debug)]
struct Entry {
    lines: Vec<Vec<u8>>,
    collumns: Vec<Vec<u8>>,
    w: usize,
    h: usize,
}

impl Entry {
    fn hash(data: &[u8]) -> u64 {
        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        hasher.finish()
    }

    fn symetry_index(v: &Vec<Vec<u8>>) -> i32 {
        let hashes: Vec<u64> = v.iter().map(|v| Entry::hash(v)).collect();

        for i in 1..hashes.len() {
            let mut all_ok: bool = true;

            for j in 1..i + 1 {
                let a = hashes.get(i - j).unwrap_or(&0);
                let b = hashes.get(i + j - 1).unwrap_or(&0);

                if (*a != 0) && (*b != 0) && *a != *b {
                    all_ok = false;
                    break;
                }
            }
            if all_ok {
                return i as i32;
            }
        }

        0
    }

    fn count_diff(a: &[u8], b: &[u8]) -> Option<usize> {
        if a.len() != b.len() {
            return None;
        }
        Some(a.iter().zip(b).filter(|(a, b)| *a != *b).count())
    }

    fn axis_with_exactly_one_smudge(v: &Vec<Vec<u8>>) -> i32 {
        for i in 1..v.len() {
            let mut sum_diff: usize = 0;

            for j in 1..i + 1 {
                let a = v.get(i - j);
                let b = v.get(i + j - 1);

                if a.is_some() && b.is_some() {
                    if let Some(c) = Entry::count_diff(a.unwrap(), b.unwrap()) {
                        sum_diff += c;
                    }
                }
            }

            if sum_diff == 1 {
                return i as i32;
            }
        }

        0
    }

    fn score(&self) -> u64 {
        let line = Entry::symetry_index(&self.lines);
        let col = Entry::symetry_index(&self.collumns);

        col as u64 + 100 * line as u64
    }

    fn score_one_smudge(&self) -> u64 {
        let line = Entry::axis_with_exactly_one_smudge(&self.lines);
        let col = Entry::axis_with_exactly_one_smudge(&self.collumns);

        col as u64 + 100 * line as u64
    }

    fn new(data: &str) -> Entry {
        let mut ret = Entry {
            lines: Vec::new(),
            collumns: Vec::new(),
            w: 0,
            h: 0,
        };

        for l in data.split('\n') {
            let ldata = l.trim().as_bytes().to_vec();
            if ret.w == 0 {
                ret.w = ldata.len();
            } else {
                assert!(ret.w == ldata.len());
            }
            ret.lines.push(ldata);
            ret.h += 1;
        }

        /* Now we have loaded the lines we want to generate the collums */
        for i in 0..ret.w {
            let cdata = ret.lines.iter().map(|v| v[i]).collect::<Vec<u8>>();
            ret.collumns.push(cdata);
        }

        ret
    }
}

fn main() {
    let file = File::open("data.txt").unwrap();
    let mut r = BufReader::new(file);
    let mut data = String::new();

    r.read_to_string(&mut data).unwrap();

    let mut entries: Vec<Entry> = Vec::new();

    for dat in data.split("\n\n") {
        entries.push(Entry::new(dat));
    }

    let sum: u64 = entries.iter().map(|v| v.score()).sum();
    let sumsmudge: u64 = entries.iter().map(|v| v.score_one_smudge()).sum();

    println!("Part 1 : {} Part 2 : {}", sum, sumsmudge);
}
