use std::fs::File;
use std::io::{BufRead, BufReader};

struct Scan {
    array: Vec<Vec<i64>>,
}

impl Scan {
    fn new() -> Scan {
        Scan { array: Vec::new() }
    }

    fn push(&mut self, data: &str) {
        let mut numbers: Vec<i64> = Vec::new();

        for v in data.split(' ') {
            let n = v.trim().parse::<i64>().unwrap();
            numbers.push(n);
        }

        self.array.push(numbers);
    }

    fn fold(line: &[i64]) -> Vec<i64> {
        let mut ret: Vec<i64> = Vec::new();
        for i in 0..line.len() - 1 {
            ret.push(line[i + 1] - line[i]);
        }

        ret
    }

    fn is_zeroes(line: &[i64]) -> bool {
        line.iter().filter(|v| **v == 0).count() == line.len()
    }

    fn predict(line: &[i64], backwards: bool) -> i64 {
        let mut preds: Vec<Vec<i64>> = Vec::new();

        let mut vline = line.to_vec();

        if backwards {
            vline.reverse();
        }

        preds.push(vline);

        loop {
            let last = preds.last().unwrap();
            let next = Scan::fold(last);
            let brk = Scan::is_zeroes(&next);
            preds.push(next);
            if brk {
                break;
            }
        }

        /* We now have unfolded the list */

        let pred = preds.iter().map(|v| v.last().unwrap()).sum();
        println!("{:?} == {}", preds, pred);
        pred
    }

    fn sum_of_preds(&self, backwards: bool) -> i64 {
        self.array.iter().map(|v| Scan::predict(v, backwards)).sum()
    }
}

fn main() {
    let f = File::open("data.txt").unwrap();
    let mut r = BufReader::new(f);

    let mut data = String::new();

    let mut sc = Scan::new();

    while let Ok(l) = r.read_line(&mut data) {
        if l == 0 {
            break;
        }

        sc.push(data.as_str());

        data.clear();
    }

    println!(
        "Part 1 : {} && Part 2 : {}",
        sc.sum_of_preds(false),
        sc.sum_of_preds(true)
    );
}
