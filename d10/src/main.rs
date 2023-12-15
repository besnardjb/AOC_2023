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

    fn transpose(v: &[Vec<Option<(i64, i64)>>]) -> Vec<Vec<Option<(i64, i64)>>> {
        /* Now transpose and redo */
        let mut t_map: Vec<Vec<Option<(i64, i64)>>> = Vec::new();

        for i in 0..v[0].len() {
            let l: Vec<Option<(i64, i64)>> = v.iter().map(|v| v[i]).collect();
            t_map.push(l);
        }

        t_map
    }

    fn print_dir(d: Option<(i64, i64)>) {
        let v = match d.unwrap_or((-9, -9)) {
            (-9, -9) => ".",
            (-2, -2) => "◦",
            (-1, 0) => "◂",
            (1, 0) => "▸",
            (0, 1) => "▾",
            (0, -1) => "▴",
            _ => "?",
        };
        print!("{}", v);
    }

    fn print_dir_field(loop_map: &[Vec<Option<(i64, i64)>>]) {
        println!("====");

        for l in loop_map.iter() {
            for v in l.iter() {
                Map::print_dir(*v);
            }
            println!();
        }
    }

    fn filter_holes(i: &[Vec<Option<(i64, i64)>>]) -> Vec<Vec<Option<(i64, i64)>>> {
        let mut ret: Vec<Vec<Option<(i64, i64)>>> = Vec::new();

        for l in i.iter() {
            let line: Vec<Option<(i64, i64)>> = l
                .iter()
                .map(|v| match v {
                    Some((-2, -2)) => Some((-2, -2)),
                    _ => None,
                })
                .collect();
            ret.push(line);
        }

        ret
    }

    fn count_holes(i: &[Vec<Option<(i64, i64)>>]) -> usize {
        let mut ret = 0;

        for l in i.iter() {
            let line = l
                .iter()
                .filter(|v| v.is_some())
                .filter(|v| v.unwrap() == (-2, -2))
                .count();
            ret += line;
        }

        ret
    }

    fn print_bool_field(loop_map: &[Vec<bool>]) {
        for l in loop_map.iter() {
            for v in l.iter() {
                print!("{}", *v as i32);
            }
            println!();
        }
    }

    fn find_area(&self) -> usize {
        let mut loop_map: Vec<Vec<Option<(i64, i64)>>> = Vec::new();

        for _ in 0..self.h {
            let v: Vec<Option<(i64, i64)>> = vec![None; self.w as usize];
            loop_map.push(v);
        }

        /* Now mark the loop */
        let lp = self.find_loop();

        for p in lp.iter() {
            if let Some(l) = loop_map.get_mut(p.1 as usize) {
                if let Some(v) = l.get_mut(p.0 as usize) {
                    *v = Some(p.2);
                }
            }
        }

        Map::print_dir_field(&loop_map);

        let mut dir1_map = loop_map.clone();

        for l in dir1_map.iter_mut() {
            let mut in_loop = false;
            let mut last_dir: (i64, i64) = (-10, -10);
            for v in l.iter_mut() {
                if v.is_some() {
                    let dir = v.unwrap();
                    if dir != last_dir {
                        in_loop = !in_loop;
                        last_dir = dir;
                    }
                }

                if in_loop && v.is_none() {
                    *v = Some((-2, -2));
                }
            }
        }

        Map::print_dir_field(&dir1_map);

        let mut loop_map = Map::transpose(&loop_map);

        for l in loop_map.iter_mut() {
            let mut in_loop = false;
            let mut last_dir: (i64, i64) = (-10, -10);
            for v in l.iter_mut() {
                if v.is_some() {
                    let dir = v.unwrap();
                    if dir != last_dir {
                        in_loop = !in_loop;
                        last_dir = dir;
                    }
                }

                if in_loop && v.is_none() {
                    *v = Some((-2, -2));
                }
            }
        }

        let dir2_map = Map::transpose(&loop_map);

        Map::print_dir_field(&dir2_map);

        /* Now mix the two directions */
        let mut dir1_holes = Map::filter_holes(&dir1_map);
        let dir2_holes = Map::filter_holes(&dir2_map);

        for (x, l) in dir1_holes.iter_mut().enumerate() {
            for (y, v) in l.iter_mut().enumerate() {
                let dir2val = dir2_holes.get(x).unwrap().get(y).unwrap();
                if v != dir2val {
                    *v = None;
                }
            }
        }

        Map::print_dir_field(&dir1_holes);

        Map::count_holes(&dir1_holes)
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
