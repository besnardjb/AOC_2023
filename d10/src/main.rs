use std::fs::File;
use std::io::{BufReader, Read};

struct Map {
    d: Vec<Vec<u8>>,
    w: i64,
    h: i64,
}

impl Map {
    fn new(data: &str) -> Map {
        let mut ret = Map {
            d: Vec::new(),
            w: 0,
            h: 0,
        };

        for l in data.split('\n') {
            let ll = l.as_bytes().to_vec();

            if ret.w == 0 {
                ret.w = ll.len() as i64;
            } else {
                assert!(ret.w as usize == ll.len());
            }

            ret.d.push(ll);

            ret.h += 1;
        }

        ret
    }

    fn get(&self, x: i64, y: i64) -> Option<u8> {
        if x < 0 || y < 0 {
            return None;
        }

        let x = x as usize;
        let y = y as usize;

        if let Some(l) = self.d.get(y) {
            if let Some(v) = l.get(x) {
                return Some(*v);
            }
        }

        None
    }

    fn walk(&self, cur: (i64, i64), dir: (i64, i64), points: &mut Vec<(i64, i64)>) -> bool {
        let v = self.get(cur.0, cur.1);

        if v.is_none() {
            return false;
        }

        let v = v.unwrap();

        if v == b'.' {
            return false;
        }

        if v == b'S' {
            return true;
        }

        /*
        | is a vertical pipe connecting north and south.
        - is a horizontal pipe connecting east and west.
        L is a 90-degree bend connecting north and east.
        J is a 90-degree bend connecting north and west.
        7 is a 90-degree bend connecting south and west.
        F is a 90-degree bend connecting south and east.
        . is ground; there is no pipe in this tile.
        */
        let mut next = cur;
        let mut new_dir = dir;

        match v {
            b'|' => {
                /* We sum the direction on Y */
                next.1 += dir.1;
            }
            b'-' => {
                /* We sum the direction on X */
                next.0 += dir.0;
            }
            b'L' => {
                /* We turn 90° */
                match dir {
                    (0, 1) => {
                        new_dir = (1, 0);
                        next.0 += new_dir.0;
                    }
                    (-1, 0) => {
                        new_dir = (0, -1);
                        next.1 += new_dir.1;
                    }
                    /* This is because S may have such value close */
                    _ => return false,
                }
            }
            b'J' => {
                /* We turn 90° */
                match dir {
                    (1, 0) => {
                        new_dir = (0, -1);
                        next.1 += new_dir.1;
                    }
                    (0, 1) => {
                        new_dir = (-1, 0);
                        next.0 += new_dir.0;
                    }
                    /* This is because S may have such value close */
                    _ => return false,
                }
            }
            b'7' => {
                /* We turn 90° */
                match dir {
                    (1, 0) => {
                        new_dir = (0, 1);
                        next.1 += new_dir.1;
                    }
                    (0, -1) => {
                        new_dir = (-1, 0);
                        next.0 += new_dir.0;
                    }
                    /* This is because S may have such value close */
                    _ => return false,
                }
            }
            b'F' => {
                /* We turn 90° */
                match dir {
                    (0, -1) => {
                        new_dir = (1, 0);
                        next.0 += new_dir.0;
                    }
                    (-1, 0) => {
                        new_dir = (0, 1);
                        next.1 += new_dir.1;
                    }
                    _ => unreachable!(),
                }
            }
            /* This is because S may have such value close */
            _ => return false,
        }

        /*println!(
            "Cur {:?} == {} Next {:?} Dir {:?}",
            cur, v as char, next, dir
        );*/

        if self.walk(next, new_dir, points) {
            points.push(next);
            return true;
        }

        false
    }

    fn start(&self) -> Option<(i64, i64)> {
        for x in 0..self.w {
            for y in 0..self.h {
                if let Some(v) = self.get(x, y) {
                    if v == b'S' {
                        return Some((x, y));
                    }
                }
            }
        }

        None
    }

    fn find_loop(&self) -> Vec<(i64, i64)> {
        let mut ret: Vec<(i64, i64)> = Vec::new();

        let s = self.start();

        if s.is_none() {
            return ret;
        }

        let s = s.unwrap();

        for x in -1..2 {
            for y in -1..3 {
                ret.clear();
                if x != 0 || y != 0 {
                    let dir = (x, y);
                    self.walk((s.0 + x, s.1 + y), dir, &mut ret);

                    if !ret.is_empty() {
                        return ret;
                    }
                }
            }
        }

        ret
    }

    fn find_area(&self) -> u64 {
        let mut area = 0;
        let mut loop_map: Vec<Vec<bool>> = Vec::new();

        for _ in 0..self.h {
            let v: Vec<bool> = vec![false; self.w as usize];
            loop_map.push(v);
        }

        /* Now mark the loop */
        let lp = self.find_loop();

        for p in lp.iter() {
            if let Some(l) = loop_map.get_mut(p.1 as usize) {
                if let Some(v) = l.get_mut(p.0 as usize) {
                    *v = true;
                }
            }
        }

        let vert_scan = loop_map.clone();

        for (y, l) in vert_scan.iter().enumerate() {
            let mut in_loop = false;
            for (x, v) in l.iter().enumerate() {
                let val = self.get(x as i64, y as i64).unwrap();
                println!("({}, {}) = {}", x, y, val as char);

                if !in_loop && *v {
                    in_loop = true;
                }
                if in_loop && !*v {
                    in_loop = false;
                }

                if val == b'.' && in_loop {
                    area += 1;
                    println!("{} {} in scan horiz", x, y);
                }
            }
        }

        println!("{:?}", loop_map);

        area
    }
}

fn main() {
    let f = File::open("data.txt").unwrap();
    let mut r = BufReader::new(f);

    let mut data = String::new();
    r.read_to_string(&mut data).unwrap();

    let map = Map::new(&data);

    let l = map.find_loop();

    println!("Part 1 half len = {}", l.len() / 2 + 1);

    map.find_area();
}
