use std::fs::File;
use std::io::{BufReader, Read};
use std::usize;

use array2d::Array2D;

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

    fn walk(
        &self,
        cur: (i64, i64),
        dir: (i64, i64),
        points: &mut Vec<(i64, i64, (i64, i64))>,
    ) -> bool {
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
                    _ => return false,
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
            points.push((next.0, next.1, new_dir));
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

    fn find_loop(&self) -> Vec<(i64, i64, (i64, i64))> {
        let mut ret: Vec<(i64, i64, (i64, i64))> = Vec::new();

        let s = self.start();

        if s.is_none() {
            return ret;
        }

        let s = s.unwrap();

        for x in -1..2 {
            for y in -1..2 {
                ret.clear();
                if x != 0 || y != 0 {
                    let dir = (x, y);
                    self.walk((s.0 + x, s.1 + y), dir, &mut ret);
                    if !ret.is_empty() {
                        ret.push((s.0 + x, s.1 + y, dir));

                        return ret;
                    }
                }
            }
        }

        ret
    }

    fn print_2d_map(loop_map: &Array2D<u8>) {
        println!("====");

        for x in 0..loop_map.column_len() + 1 {
            for y in 0..loop_map.row_len() + 1 {
                if let Some(v) = loop_map.get(x, y) {
                    print!("{}", *v as char);
                }
            }
            println!();
        }
    }

    fn find_area(&self) -> usize {
        println!("{} x {}", self.w, self.h);

        let mut prefilled = Array2D::filled_with(b'.', self.h as usize, self.w as usize);

        /* Map the loop in the array */
        for lp in self.find_loop() {
            prefilled
                .set(
                    lp.1 as usize,
                    lp.0 as usize,
                    self.d[lp.1 as usize][lp.0 as usize],
                )
                .unwrap();
        }

        Map::print_2d_map(&prefilled);
        let vert_entries = [b'F', b'7', b'J', b'L', b'S', b'|'];

        let mut inside_list: Vec<(usize, usize)> = Vec::new();

        /* Over rows */
        for (x, row) in prefilled.rows_iter().enumerate() {
            let mut inside = false;

            for (y, v) in row.enumerate() {
                let mut ignored = true;

                let prev = if y >= 1 {
                    prefilled.get(x, y - 1).unwrap()
                } else {
                    &b'.'
                };

                let next = if y < prefilled.column_len() - 1 {
                    prefilled.get(x, y + 1).unwrap()
                } else {
                    &b'.'
                };

                if *prev == b'.' || *next == b'.' {
                    ignored = false;
                }

                if vert_entries.contains(v) && !ignored {
                    inside = !inside;
                }

                println!(
                    "{} ({} {}) {}",
                    *v as char, *prev as char, *next as char, inside
                );

                if *v == b'.' && inside {
                    inside_list.push((x, y));
                }
            }
        }

        /* No inside can be part of a non ground value */
        let inside_list: Vec<(usize, usize)> = inside_list
            .iter()
            .filter(|v| self.d[v.0][v.1] == b'.')
            .copied()
            .collect();

        for p in inside_list {
            prefilled.set(p.0, p.1, b'H').unwrap();
        }

        Map::print_2d_map(&prefilled);

        prefilled
            .elements_row_major_iter()
            .filter(|v| **v == b'H')
            .count()
    }
}

fn main() {
    let f = File::open("data.txt").unwrap();
    let mut r = BufReader::new(f);

    let mut data = String::new();
    r.read_to_string(&mut data).unwrap();

    let map = Map::new(&data);

    let l = map.find_loop();

    println!(
        "Part 1 half len = {} Part 2 area: {}",
        l.len() / 2 + 1,
        map.find_area()
    );
}
