use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

use anyhow::Error as Aerr;

struct Array2D {
    lines: Vec<Vec<u8>>,
    w: usize,
    h: usize,
}

impl Array2D {
    fn new() -> Array2D {
        Array2D {
            lines: Vec::new(),
            w: 0,
            h: 0,
        }
    }

    fn add_line(&mut self, data: &String) -> Result<(), Aerr> {
        if self.w == 0 {
            self.w = data.len();
        } else if self.w != data.len() {
            return Err(Aerr::msg(format!(
                "Bad line lenght {} != {}\n{}",
                data.len(),
                self.w,
                data
            )));
        }

        let data: Vec<u8> = data.clone().into_bytes();

        self.lines.push(data);

        self.h += 1;

        Ok(())
    }

    fn get_gears(&self) -> Vec<(usize, usize)> {
        let mut ret: Vec<(usize, usize)> = Vec::new();

        for (x, l) in self.lines.iter().enumerate() {
            for (y, v) in l.iter().enumerate() {
                if *v == b'*' {
                    ret.push((x, y));
                }
            }
        }

        ret
    }

    fn get_locators_close_to(
        locators: &[(usize, usize, usize)],
        coord: &(usize, usize),
    ) -> Vec<(usize, usize, usize)> {
        let mut ret: Vec<(usize, usize, usize)> = Vec::new();

        for loc in locators.iter() {
            for dx in -1..2 {
                for dy in -1..2 {
                    if (((coord.0 as i32 + dx) as usize) == loc.0)
                        && (loc.1 as i32 <= (coord.1 as i32 + dy))
                        && ((coord.1 as i32 + dy) < (loc.1 + loc.2) as i32)
                    {
                        ret.push(*loc);
                        break;
                    }
                }
            }
        }

        ret
    }

    fn get_at(&self, x: usize, y: usize) -> Option<u8> {
        if let Some(line) = self.lines.get(x) {
            if let Some(val) = line.get(y) {
                return Some(*val);
            }
        }
        None
    }

    fn get_numbers_locators(&self) -> Vec<(usize, usize, usize)> {
        let mut ret: Vec<(usize, usize, usize)> = Vec::new();

        for (x, l) in self.lines.iter().enumerate() {
            let mut y = 0;
            while y < self.w {
                let v = l[y];
                if v.is_ascii_digit() {
                    let mut vlen = 1;
                    /* Get value length */
                    let mut yy = y + 1;
                    while let Some(vv) = l.get(yy) {
                        if vv.is_ascii_digit() {
                            vlen += 1;
                        } else {
                            break;
                        }
                        yy += 1;
                    }
                    ret.push((x, y, vlen));
                    /* Skip found number */
                    y += vlen;
                }
                y += 1;
            }
        }

        ret
    }

    fn locator_to_number(&self, desc: &(usize, usize, usize)) -> Option<i32> {
        if let Some(line) = self.lines.get(desc.0) {
            if let Ok(val) = String::from_utf8(line[desc.1..desc.1 + desc.2].to_vec()) {
                if let Ok(v) = val.parse::<i32>() {
                    return Some(v);
                }
            }
        }

        None
    }

    fn has_surounding_symbol(&self, x: usize, y: usize) -> bool {
        for dx in -1..2 {
            for dy in -1..2 {
                let xx: i32 = x as i32 + dx;
                let yy: i32 = y as i32 + dy;

                if (xx < 0) || (yy < 0) {
                    continue;
                }

                if let Some(v) = self.get_at(xx as usize, yy as usize) {
                    if v == b'.' || v.is_ascii_digit() {
                        continue;
                    }
                    return true;
                }
            }
        }

        false
    }

    fn check_number_has_surrounding(&self, desc: (usize, usize, usize)) -> bool {
        let x = desc.0;
        for y in desc.1..(desc.1 + desc.2) {
            if self.has_surounding_symbol(x, y) {
                return true;
            }
        }

        false
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("./data.txt")?;
    let mut r = BufReader::new(f);

    let mut data = String::new();

    let mut array = Array2D::new();

    while let Ok(len) = r.read_line(&mut data) {
        if len == 0 {
            break;
        }

        if data.ends_with('\n') {
            data = String::from(&data[0..data.len() - 1]);
        }

        array.add_line(&data)?;

        data.clear();
    }

    let nums = array.get_numbers_locators();

    let nums_with_symb: Vec<i32> = nums
        .iter()
        .filter(|v| array.check_number_has_surrounding(**v))
        .filter_map(|v| array.locator_to_number(v))
        .collect();

    let sum: i32 = nums_with_symb.iter().sum();

    println!("Sum {:?}", sum);

    let gears = array.get_gears();

    let mut prod_sum = 0;

    for g in gears.iter() {
        let matches_prod: Vec<i32> = Array2D::get_locators_close_to(&nums, g)
            .iter()
            .filter_map(|v| array.locator_to_number(v))
            .collect();

        if matches_prod.len() > 1 {
            let prod: i32 = matches_prod.iter().product();
            prod_sum += prod;
        }
    }

    println!("Prod sum {}", prod_sum);

    Ok(())
}
