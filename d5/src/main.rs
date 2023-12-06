use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};

fn load_num_list(data: &str) -> Vec<i64> {
    let mut ret: Vec<i64> = Vec::new();

    for v in data.trim().split(' ') {
        if let Ok(v) = v.parse::<i64>() {
            ret.push(v);
        }
    }

    ret
}

struct LookupEntry {
    source: i64,
    dest: i64,
    len: i64,
}

impl LookupEntry {
    fn new(dest: i64, source: i64, len: i64) -> LookupEntry {
        LookupEntry { source, dest, len }
    }

    fn resolve(&self, input: &i64) -> Option<i64> {
        if (self.source <= *input) && (*input < (self.source + self.len)) {
            return Some(self.dest + (input - self.source));
        }

        None
    }
}

struct Lookup {
    from: String,
    to: String,
    lookups: Vec<LookupEntry>,
}

impl Lookup {
    fn new(data: &str) -> Lookup {
        let lines: Vec<&str> = data.split('\n').collect();

        assert!(lines[0].contains(" map:"));

        let set_desc = lines[0][..lines[0].len() - " map:".len()].to_string();
        let set_desc: Vec<&str> = set_desc.split("-to-").collect();
        assert!(set_desc.len() == 2);

        let from = set_desc[0].trim().to_string();
        let to = set_desc[1].trim().to_string();

        let mut lookups: Vec<LookupEntry> = Vec::new();

        for v in lines[1..].iter() {
            let vals = load_num_list(v);
            assert!(vals.len() == 3);

            lookups.push(LookupEntry::new(vals[0], vals[1], vals[2]));
        }

        Lookup { from, to, lookups }
    }

    fn resolve(&self, from: &i64) -> i64 {
        for l in self.lookups.iter() {
            if let Some(v) = l.resolve(from) {
                return v;
            }
        }

        *from
    }
}

fn get_seeds(data: &str) -> Vec<i64> {
    assert!(data.starts_with("seeds:"));

    let colon = data.find(':').unwrap();

    let numlist = data[colon + 1..].to_string();

    load_num_list(&numlist)
}

fn main() {
    let f = File::open("data.txt").unwrap();
    let mut r = BufReader::new(f);
    let mut data = String::new();
    r.read_to_string(&mut data).unwrap();

    let fields: Vec<&str> = data.split("\n\n").collect();

    assert!(fields.len() > 1);

    let mut converters: HashMap<String, Lookup> = HashMap::new();

    for e in fields[1..].iter() {
        let look = Lookup::new(e);
        converters.insert(look.from.to_string(), look);
    }

    /* Part 1 */

    let mut values = get_seeds(fields[0]);
    println!("{:?}", values);

    let mut current_target = "seed";

    while let Some(lk) = converters.get(current_target) {
        println!("{} -> {}", lk.from, lk.to);
        println!("{:?}", values);
        values = values.iter().map(|v| lk.resolve(v)).collect();
        println!("{:?}", values);
        current_target = lk.to.as_str();
    }

    println!("Part 1 min is {}", values.iter().min().unwrap());

    /* Part 2 */

    let values = get_seeds(fields[0]);

    assert!(values.len() % 2 == 0);

    let mut smin: i64 = 0;
    let mut k = 0;

    while k < values.len() {
        let mut left = values[k + 1];
        for seed in values[k]..values[k + 1] + values[k] {
            //println!("s! {}", seed);
            let mut values = vec![seed];
            let mut current_target = "seed";

            while let Some(lk) = converters.get(current_target) {
                //println!("{} -> {}", lk.from, lk.to);
                values = values.iter().map(|v| lk.resolve(v)).collect();
                current_target = lk.to.as_str();
            }

            if smin == 0 || (values[0] < smin) {
                smin = values[0];
            }
            left -= 1;
            if (left % 1000000) == 0 {
                println!("Left {}", left)
            }
        }
        println!("Seed {} done", values[k]);

        k += 2;
    }

    println!("Part 2 min is {}", smin);
}
