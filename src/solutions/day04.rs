use std::collections::HashMap;
use crate::puzzle::Puzzle;

impl Puzzle {
    pub fn day04(&mut self) {
        fn between<T: std::cmp::PartialOrd>(value: T, low: T, high: T) -> bool { (low <= value) && (value <= high) }
        
        let mut valid_count_1 = 0;
        let mut valid_count_2 = 0;

        let mut passport = HashMap::new();
        let mut input_iter = self.input.iter();

        loop {
            let line = input_iter.next();
            if line == Some(&String::from("")) || line == None {
                if passport.len() == (7 + passport.contains_key("cid") as usize) {
                    valid_count_1 += 1;
                    valid_count_2 += passport.into_iter().all(|(key,val):(&str,&str)| -> bool {match key {
                        "cid" => true,
                        "byr" => between(val.parse::<i32>().unwrap_or(0), 1920, 2002),
                        "iyr" => between(val.parse::<i32>().unwrap_or(0), 2010, 2020),
                        "eyr" => between(val.parse::<i32>().unwrap_or(0), 2020, 2030),
                        "ecl" => ["amb","blu","brn","gry","grn","hzl","oth"].into_iter().any(|x| x==val),
                        "pid" => (val.chars().count() == 9) && (val.parse::<i32>().is_ok()),
                        "hgt" => {
                            if let Some((idx,_)) = val.char_indices().rev().nth(1) {
                                match val.get(idx..) {
                                    Some("in") => between(val.get(0..idx).and_then(|x| x.parse::<i32>().ok()).unwrap_or(0),  59,  76),
                                    Some("cm") => between(val.get(0..idx).and_then(|x| x.parse::<i32>().ok()).unwrap_or(0), 150, 193),
                                    _ => false,
                                }
                            } else { false }
                        }
                        "hcl" => {
                            let mut chrs = val.chars();
                            (chrs.next() == Some('#')) && chrs.all(|x| "0123456789abcdef".chars().any(|y| x==y))
                        }
                        key => {
                            println!("Unknown attribute {}:{}",key,val);
                            false
                        },
                    }}) as i32;
                }
                if line == None { break; }
                passport = HashMap::new();

            } else {  
                for item in line.unwrap().split(' ').map(|x| x.split(':').collect::<Vec<&str>>()) {
                    passport.insert(item[0],item[1]);
                }
            }
        }
        self.set_answer_a(valid_count_1);
        self.set_answer_b(valid_count_2);
    }
}