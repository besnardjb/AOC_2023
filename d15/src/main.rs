use std::fs::File;
use std::io::{BufReader, Read};

struct Entry {
    d: Vec<u8>,
    h: u64,
}

impl Entry {
    fn hash(d: &[u8]) -> u64 {
        let mut ret: u64 = 0;

        for v in d.iter() {
            let vv = *v as u64;
            ret += vv;
            ret *= 17;
            ret %= 256;
        }
        ret
    }

    fn new(d: &str) -> Entry {
        let mut ret = Entry {
            d: d.as_bytes().to_vec(),
            h: 0,
        };

        ret.h = Entry::hash(&ret.d);

        ret
    }
}

struct Box {
    entries: Vec<(String, u64)>,
}

impl Box {
    fn new() -> Box {
        Box {
            entries: Vec::new(),
        }
    }

    fn set(&mut self, label: &str, val: u64) {
        for v in self.entries.iter_mut() {
            if v.0 == label {
                v.1 = val;
                return;
            }
        }

        self.entries.push((label.to_string(), val));
    }

    fn remove(&mut self, label: &str) {
        self.entries.retain(|v| v.0 != label);
    }

    fn score(&self, bx: u64) -> u64 {
        let mut ret: u64 = 0;

        for (x, v) in self.entries.iter().enumerate() {
            ret += bx * (x as u64 + 1) * v.1;
        }

        ret
    }
}

struct Boxes {
    bxs: Vec<Box>,
}

impl Boxes {
    fn new() -> Boxes {
        let mut bxs: Vec<Box> = Vec::with_capacity(256);

        for _ in 0..256 {
            let b = Box::new();
            bxs.push(b);
        }

        Boxes { bxs }
    }

    fn insert(&mut self, e: &Entry) {
        let (op, key, value) = if e.d.ends_with(&[b'-']) {
            (
                '-',
                String::from_utf8(e.d[..e.d.len() - 1].to_vec()).unwrap(),
                0,
            )
        } else {
            let s = String::from_utf8(e.d.clone()).unwrap();
            let ss: Vec<&str> = s.split('=').collect();
            ('=', ss[0].to_string(), ss[1].parse::<u64>().unwrap())
        };

        let e = Entry::new(&key);

        match op {
            '-' => {
                self.bxs[e.h as usize].remove(&key);
            }
            '=' => {
                self.bxs[e.h as usize].set(&key, value);
            }
            _ => unreachable!(),
        }
    }

    fn score(&self) -> u64 {
        self.bxs
            .iter()
            .enumerate()
            .map(|(x, v)| v.score(x as u64 + 1))
            .sum()
    }
}

fn main() {
    let f = File::open("data.txt").unwrap();
    let mut r = BufReader::new(f);

    let mut data: String = String::new();

    let mut entries: Vec<Entry> = Vec::new();

    r.read_to_string(&mut data).unwrap();

    for e in data.split(',') {
        let entry = Entry::new(e.trim());
        entries.push(entry);
    }

    let sum: u64 = entries.iter().map(|v| v.h).sum();

    println!("Part 1 Sum {}", sum);

    let mut boxes = Boxes::new();

    for e in entries.iter() {
        boxes.insert(e);
    }

    println!("Part 2 Score is {}", boxes.score());
}
