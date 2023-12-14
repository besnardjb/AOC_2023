use std::fs::File;
use std::io::{BufReader, Read};

#[derive(Debug)]
struct Entry {
    lines: Vec<Vec<u8>>,
    collumns: Vec<Vec<u8>>,
    w: usize,
    h: usize,
}

fn abs(v: i64) -> i64 {
    if v < 0 {
        return -v;
    }
    v
}

impl Entry {
    fn is_empty(v: &[u8]) -> bool {
        v.len() == v.iter().filter(|v| **v == b'.').count()
    }

    fn empty_offsets(v: &[Vec<u8>]) -> Vec<usize> {
        v.iter()
            .enumerate()
            .filter_map(|(i, v)| if Entry::is_empty(v) { Some(i) } else { None })
            .collect()
    }

    fn galaxies(&self, expand_factor: usize) -> Vec<(usize, usize)> {
        let mut ret: Vec<(usize, usize)> = Vec::new();

        for (x, l) in self.lines.iter().enumerate() {
            for (y, v) in l.iter().enumerate() {
                if *v == b'#' {
                    ret.push((x, y));
                }
            }
        }

        /* We have the list of galaxies */
        let list_of_empty_lines: Vec<usize> = Entry::empty_offsets(&self.lines);
        let list_of_empty_cols: Vec<usize> = Entry::empty_offsets(&self.collumns);

        for g in ret.iter_mut() {
            let lower_x: usize = list_of_empty_lines.iter().filter(|v| **v < g.0).count();
            let lower_y: usize = list_of_empty_cols.iter().filter(|v| **v < g.1).count();

            g.0 += lower_x * expand_factor;
            g.1 += lower_y * expand_factor;
        }

        ret
    }

    fn rotate(v: &[Vec<u8>]) -> Vec<Vec<u8>> {
        let mut ret: Vec<Vec<u8>> = Vec::new();

        let len = v[0].len();

        /* Now we have loaded the lines we want to generate the collums */
        for i in 0..len {
            let cdata = v.iter().map(|vv| vv[i]).collect::<Vec<u8>>();
            ret.push(cdata);
        }

        ret
    }

    fn distance(a: &(usize, usize), b: &(usize, usize)) -> usize {
        (abs(b.0 as i64 - a.0 as i64) + abs(b.1 as i64 - a.1 as i64)) as usize
    }

    fn sum_of_distances(&self, expand_factor: usize) -> usize {
        let mut ret: usize = 0;
        let gal = self.galaxies(expand_factor);

        for i in 0..gal.len() {
            for j in i + 1..gal.len() {
                ret += Entry::distance(&gal[i], &gal[j]);
            }
        }

        ret
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

        ret.collumns = Entry::rotate(&ret.lines);

        ret
    }
}

fn main() {
    let f = File::open("data.txt").unwrap();
    let mut r = BufReader::new(f);

    let mut data = String::new();
    r.read_to_string(&mut data).unwrap();

    let e = Entry::new(&data);

    println!(
        "Part 1 {} Part 2 {}",
        e.sum_of_distances(1),
        e.sum_of_distances(1000000 - 1)
    );
}
