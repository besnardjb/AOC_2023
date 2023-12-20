use array2d::Array2D;
use rayon::prelude::*;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, Read};

struct Map {
    m: Array2D<u8>,
}

impl Map {
    fn new(data: &str) -> Map {
        let l: Vec<Vec<u8>> = data.split('\n').map(|v| v.as_bytes().to_vec()).collect();

        let m = Array2D::from_rows(&l).unwrap();
        Map { m }
    }

    fn walk(&self, x: i64, y: i64, dir: (i64, i64)) -> usize {
        let mut e: Array2D<bool> = self.clear_energy();
        let mut visited: HashSet<String> = HashSet::new();
        self._walk(x, y, dir, &mut e, &mut visited);
        Map::energized(&e)
    }

    fn _walk(
        &self,
        x: i64,
        y: i64,
        dir: (i64, i64),
        energy: &mut Array2D<bool>,
        visited: &mut HashSet<String>,
    ) {
        if x < 0 || y < 0 {
            return;
        }

        let cur = self.m.get(x as usize, y as usize);

        if cur.is_none() {
            return;
        }

        let cur = cur.unwrap();

        let visit = format!("{:?}x{}y{}", dir, x, y);

        if visited.contains(&visit) {
            return;
        }

        /* Set cell as energized */
        energy.set(x as usize, y as usize, true).unwrap();

        visited.insert(visit);

        let new_dir = match *cur {
            b'.' => (dir, (0, 0)),
            b'/' => match dir {
                (-1, 0) => ((0, 1), (0, 0)),
                (1, 0) => ((0, -1), (0, 0)),
                (0, 1) => ((-1, 0), (0, 0)),
                (0, -1) => ((1, 0), (0, 0)),
                _ => unreachable!(),
            },
            b'\\' => match dir {
                (-1, 0) => ((0, -1), (0, 0)),
                (1, 0) => ((0, 1), (0, 0)),
                (0, 1) => ((1, 0), (0, 0)),
                (0, -1) => ((-1, 0), (0, 0)),
                _ => unreachable!(),
            },
            b'-' => match dir {
                (0, 1) | (0, -1) => (dir, (0, 0)),
                (1, 0) | (-1, 0) => ((0, -1), (0, 1)),
                _ => unreachable!(),
            },
            b'|' => match dir {
                (0, -1) | (0, 1) => ((1, 0), (-1, 0)),
                (1, 0) | (-1, 0) => (dir, (0, 0)),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        };

        let d1 = new_dir.0;
        let d2 = new_dir.1;

        self._walk(x + d1.0, y + d1.1, d1, energy, visited);

        if d2 != (0, 0) {
            self._walk(x + d2.0, y + d2.1, d2, energy, visited);
        }
    }

    fn clear_energy(&self) -> Array2D<bool> {
        Array2D::filled_with(false, self.m.num_rows(), self.m.num_columns())
    }

    fn energized(energy: &Array2D<bool>) -> usize {
        energy.elements_row_major_iter().filter(|v| **v).count()
    }

    fn scan_edges(&mut self) -> usize {
        let mut sources: Vec<((i64, i64), (i64, i64))> = Vec::new();

        for x in [0, self.m.row_len() - 1] {
            for y in 0..self.m.column_len() {
                let dir = if x == 0 { (1, 0) } else { (-1, 0) };
                sources.push(((x as i64, y as i64), dir));
            }
        }

        for y in [0, self.m.column_len() - 1] {
            for x in 0..self.m.row_len() {
                let dir = if y == 0 { (0, 1) } else { (0, -1) };
                sources.push(((x as i64, y as i64), dir));
            }
        }

        sources
            .par_iter()
            .map(|source| self.walk(source.0 .0, source.0 .1, source.1))
            .max()
            .unwrap()
    }
}

fn main() {
    let f = File::open("data.txt").unwrap();
    let mut r = BufReader::new(f);

    let mut data: String = String::new();

    r.read_to_string(&mut data).unwrap();

    let mut m = Map::new(&data);

    println!(
        "Part 1 energized {} Part 2 : {:?}",
        m.walk(0, 0, (0, 1)),
        m.scan_edges()
    );
}
