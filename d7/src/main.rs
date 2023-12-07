use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;

use std::fs::File;
use std::io::BufReader;
use std::io::Read;

#[derive(Debug, Clone)]
struct Hand {
    hand: String,
    score: u64,
    count: HashMap<u8, u32>,
    joker: bool,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Kind {
    Highcard,
    Onepair,
    Twopair,
    Threeoak,
    Fullh,
    Fouroak,
    Fiveoak,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Card {
    Jocker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

impl Hand {
    fn new(data: &str, joker: bool) -> Hand {
        let v: Vec<&str> = data.split(' ').collect();

        assert!(v.len() == 2);

        let hand = v[0].to_string();
        let score = v[1].trim().parse::<u64>().unwrap();

        let mut ret = Hand {
            hand,
            score,
            count: HashMap::new(),
            joker,
        };

        for v in ret.hand.clone().into_bytes().iter() {
            if let Some(v) = ret.count.get_mut(v) {
                *v += 1;
            } else {
                ret.count.insert(*v, 1);
            }
        }

        let cards: HashSet<u8> = ret.count.keys().cloned().collect();

        if cards.contains(&b'J') && ret.joker {
            /* We have a joker */
            let mut max_card: u8 = b'J';
            let maxi = *ret.count.values().max().unwrap();

            let mut max_non_j: u32 = 0;
            let mut max_val_non_j = b'J';

            let jcount = *ret.count.get(&b'J').unwrap();

            for (k, v) in ret.count.iter() {
                let cur = ret.to_card(k);
                let cmax = ret.to_card(&max_card);

                if maxi == *v && cur > cmax {
                    max_card = *k;
                }

                if *k != b'J' {
                    let cmaxnon_j = ret.to_card(&max_val_non_j);

                    if max_non_j == 0 || cur > cmaxnon_j {
                        max_non_j = *v;
                        max_val_non_j = *k;
                    }
                }
            }

            if max_card == b'J' {
                if maxi == 5 {
                    max_card = b'A';
                } else {
                    max_card = max_val_non_j;
                }
            }

            ret.count.remove(&b'J');

            println!("{} becomes {:?}", ret.hand, ret.to_card(&max_card));

            if let Some(v) = ret.count.get(&max_card) {
                ret.count.insert(max_card, *v + jcount);
            } else {
                ret.count.insert(max_card, jcount);
            }
        }

        ret
    }

    fn to_kind(&self) -> Kind {
        /* Recompute counts */
        let set: HashSet<u32> = self.count.values().cloned().collect();

        if set.contains(&5) {
            return Kind::Fiveoak;
        }

        if set.contains(&4) {
            return Kind::Fouroak;
        }

        if set.contains(&3) && set.contains(&2) {
            return Kind::Fullh;
        }

        if set.contains(&3) {
            return Kind::Threeoak;
        }

        let pair_count: i32 = self.count.values().filter(|v| **v == 2).map(|_| 1).sum();

        match pair_count {
            2 => Kind::Twopair,
            1 => Kind::Onepair,
            _ => Kind::Highcard,
        }
    }

    fn to_card(&self, card: &u8) -> Card {
        if self.joker && *card == b'J' {
            return Card::Jocker;
        }

        match *card {
            b'2' => Card::Two,
            b'3' => Card::Three,
            b'4' => Card::Four,
            b'5' => Card::Five,
            b'6' => Card::Six,
            b'7' => Card::Seven,
            b'8' => Card::Eight,
            b'9' => Card::Nine,
            b'T' => Card::T,
            b'J' => Card::J,
            b'Q' => Card::Q,
            b'K' => Card::K,
            b'A' => Card::A,
            _ => unreachable!("No such card {}", *card),
        }
    }

    fn is_card_lower(&self, other: &Hand) -> bool {
        //println!("{} && {}", self.hand, other.hand);
        assert!(self.joker == other.joker);

        let other = other.hand.as_bytes();

        for (i, c) in self.hand.as_bytes().iter().enumerate() {
            let loc = self.to_card(c);
            let rem = self.to_card(&other[i]);
            if loc == rem {
                //println!("{:?} == {:?}", loc, rem);
                continue;
            }

            if loc < rem {
                //println!("{:?} < {:?}", loc, rem);
                return true;
            } else {
                //println!("{:?} > {:?}", loc, rem);

                return false;
            }
        }

        false
    }
}

fn main() {
    let f = File::open("data.txt").unwrap();
    let mut r = BufReader::new(f);
    let mut data = String::new();

    r.read_to_string(&mut data).unwrap();

    let mut hands: Vec<Hand> = Vec::new();

    for v in data.split('\n') {
        hands.push(Hand::new(v, false));
    }

    hands.sort_by(|a, b| {
        let ka = a.to_kind();
        let kb = b.to_kind();

        if ka == kb {
            let alower = a.is_card_lower(b);
            match alower {
                true => Ordering::Less,
                false => Ordering::Greater,
            }
        } else {
            ka.cmp(&kb)
        }
    });

    let mut score: u64 = 0;

    for (i, j) in hands.iter().enumerate() {
        println!("{} = {}", i, j.hand);
        score += (i as u64 + 1) * j.score;
    }

    println!("Part 1 Score {}", score);

    hands.clear();
    for v in data.split('\n') {
        hands.push(Hand::new(v, true));
    }

    hands.sort_by(|a, b| {
        let ka = a.to_kind();
        let kb = b.to_kind();

        if ka == kb {
            let alower = a.is_card_lower(b);
            match alower {
                true => Ordering::Less,
                false => Ordering::Greater,
            }
        } else {
            ka.cmp(&kb)
        }
    });

    let mut score: u64 = 0;

    for (i, j) in hands.iter().enumerate() {
        println!("{} = {} {:?}", i, j.hand, j.to_kind());
        score += (i as u64 + 1) * j.score;
    }

    println!("Part 2 Score {}", score);
}
