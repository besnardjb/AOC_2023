use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::str::{from_utf8, from_utf8_unchecked};
use std::sync::Mutex;

#[derive(Debug)]
struct Puzzle {
    d: Vec<u8>,
    g: Vec<i32>,
}

impl Puzzle {
    fn new(data: &str) -> Puzzle {
        let sp: Vec<&str> = data.split(' ').collect();

        assert!(sp.len() == 2);

        Puzzle {
            d: sp[0].as_bytes().to_vec(),
            g: sp[1]
                .split(',')
                .map(|v| v.trim().parse::<i32>().unwrap())
                .collect(),
        }
    }

    fn unfold(&mut self) {
        let mut new_g: Vec<i32> = self
            .g
            .iter()
            .cycle()
            .take(self.g.len() * 5)
            .cloned()
            .collect();

        let prev = String::from_utf8(self.d.clone()).unwrap();
        let d = format!("{}?{}?{}?{}?{}", prev, prev, prev, prev, prev);

        self.d = d.as_bytes().to_vec();
        self.g = new_g;
    }

    fn is_valid(&self, e: &str) -> bool {
        let v: Vec<i32> = Puzzle::group_vec(e);
        self.g == v
    }

    fn group_vec(e: &str) -> Vec<i32> {
        e.split('.')
            .filter(|v| !v.is_empty())
            .map(|v| v.len() as i32)
            .collect()
    }

    fn _walk(
        &self,
        d: &Vec<u8>,
        off: usize,
        cache: &mut HashMap<(String, Vec<i32>), usize>,
    ) -> usize {
        let mut ret = 0;

        if !d.contains(&b'?') {
            //let v = String::from_utf8(d.clone()).unwrap();
            let valid = self.is_valid(from_utf8(d).unwrap());
            if valid {
                //println!("{} == {}", valid, v);
                return 1;
            }
        }

        let groups_vec = Puzzle::group_vec(from_utf8(&d[..off]).unwrap());

        if groups_vec.len() > self.g.len() {
            return 0;
        }

        for v in groups_vec.iter().enumerate() {
            if *v.1 != self.g[v.0] {
                return 0;
            }
        }

        let left_group = self.g[groups_vec.len()..].to_vec();

        let s = String::from_utf8(d[off..].to_vec()).unwrap();
        if let Some(prev) = cache.get(&(s.clone(), left_group.clone())) {
            println!("Hit for {} {:?} = {}", s, left_group, *prev);
            return *prev;
        }

        for x in off..d.len() {
            let v = d[x];
            if v == b'?' {
                ret += [b'.', b'#']
                    .iter()
                    .map(|v| {
                        let mut local = d.clone();
                        local[x] = *v;
                        self._walk(&local, off + 1, cache)
                    })
                    .sum::<usize>();

                break;
            }
        }

        if d[off + 1..].contains(&b'?') {
            let s = String::from_utf8(d[off..].to_vec()).unwrap();
            cache.insert((s.clone(), left_group.clone()), ret);
            //println!("{} @ {:?} == {}", s, left_group, ret);
        }

        ret
    }

    fn walk(&self) -> usize {
        let d = self.d.clone();
        let mut cache: HashMap<(String, Vec<i32>), usize> = HashMap::new();
        self._walk(&d, 0, &mut cache)
    }
}

fn main() {
    let f = File::open("data.txt").unwrap();
    let mut b = BufReader::new(f);

    let mut data: String = String::new();

    b.read_to_string(&mut data).unwrap();

    let mut puz: Vec<Puzzle> = data.split('\n').map(Puzzle::new).collect();

    println!("Part 1 : {}", puz.iter().map(|p| p.walk()).sum::<usize>());

    puz.iter_mut().for_each(|v| v.unfold());

    println!(
        "Part 2 : {}",
        puz.iter()
            .map(|p| {
                println!("Line {:?}", p);
                p.walk()
            })
            .sum::<usize>()
    );
}
