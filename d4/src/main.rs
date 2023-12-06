use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Card {
    id: i32,
    candi: HashSet<i32>,
    win: HashSet<i32>,
}

impl Card {
    fn parse_num_list(data: &str) -> Vec<i32> {
        let mut ret: Vec<i32> = Vec::new();

        for x in data.split(' ') {
            if let Ok(v) = x.trim().parse::<i32>() {
                ret.push(v);
            }
        }

        ret
    }

    fn new(data: &str) -> Card {
        assert!(data.starts_with("Card "));

        let colon = data.find(':').unwrap();

        let series = String::from(&data[colon + 1..]);
        let id = String::from(&data[5..colon]).trim().parse::<i32>().unwrap();

        let ent: Vec<&str> = series.split('|').collect();

        assert!(ent.len() == 2);

        let candi = Card::parse_num_list(ent[0]);
        let winning = Card::parse_num_list(ent[1]);

        let mut ret = Card {
            id,
            candi: HashSet::new(),
            win: HashSet::new(),
        };

        for v in candi.iter() {
            ret.candi.insert(*v);
        }

        for v in winning.iter() {
            ret.win.insert(*v);
        }

        println!("{:?} {}", ret, ret.score());

        ret
    }

    fn num_matching(&self) -> i32 {
        let mut score = 0;

        for v in self.candi.iter() {
            if self.win.contains(v) {
                score += 1;
            }
        }

        score
    }

    fn score(&self) -> i32 {
        let mut score = 0;

        for v in self.candi.iter() {
            if self.win.contains(v) {
                match score {
                    0 => score += 1,
                    _ => score *= 2,
                }
            }
        }

        score
    }
}

fn unfold_winning_cards(cards: &Vec<Card>, card: &Card) -> i32 {
    let mut ret: i32 = 0;

    ret += 1;

    for j in 1..card.num_matching() + 1 {
        if let Some(new_card) = cards.get((card.id - 1) as usize + j as usize) {
            ret += unfold_winning_cards(cards, new_card);
        }
    }

    ret
}

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("data.txt")?;

    let mut r = BufReader::new(f);

    let mut cards: Vec<Card> = Vec::new();

    let mut data = String::new();

    while let Ok(l) = r.read_line(&mut data) {
        if l == 0 {
            break;
        }

        cards.push(Card::new(&data));

        data.clear();
    }

    let total: i32 = cards.iter().map(|v| v.score()).sum();

    println!("Total score : {}", total);

    /* Now I process with Part 2 rule */
    let mut ret = 0;

    for c in cards.iter() {
        ret += unfold_winning_cards(&cards, c);
    }

    println!("Final stack len {}", ret);

    Ok(())
}
