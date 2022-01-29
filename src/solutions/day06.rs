use std::collections::HashSet;
use crate::puzzle::Puzzle;

impl Puzzle {
    pub fn day06(&mut self)  {
        const ALPHABET: [char; 26] = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];
        let mut input_iter = self.input.iter();
        let mut any_sum = 0;
        let mut all_sum = 0;
        let mut any_yes = HashSet::new();
        let mut all_yes = Vec::from(ALPHABET);
        loop {
            let line = input_iter.next();
            if line == Some(&String::from("")) || line == None {
                any_sum += any_yes.len();
                all_sum += all_yes.len();
                any_yes.clear();
                all_yes = Vec::from(ALPHABET);
                if line == None { break; }
            } else {
                line.unwrap().chars().for_each( |x| { any_yes.insert(x); } );
                all_yes = line.unwrap().chars().filter( |x| all_yes.contains(x) ).collect();
            }
        }
        self.set_answer_a(any_sum);
        self.set_answer_b(all_sum);
    }
}