use std::collections::HashMap;
use std::cmp::Ordering::{Greater, Equal};

use crate::puzzle::Puzzle;
impl Puzzle {

   
    pub fn day07(&mut self) {
        fn bag_contains_gold(rules: &HashMap<String,Vec<(i32,String)>>, bag: &str) -> bool {
            for sub_bag in rules.get(bag).unwrap_or(&Vec::new()) {
                if sub_bag.1 == "shiny gold" || bag_contains_gold(rules, &sub_bag.1) { return true; }
            }
            false
        }
        fn count_bags(rules: &HashMap<String,Vec<(i32,String)>>, bag: &str) -> i32 {
            let mut count = 1;
            for sub_bag in rules.get(bag).unwrap_or(&Vec::new()) {
                count += sub_bag.0 * count_bags(rules, &sub_bag.1);
            }
            count
        }       
        
        let mut rules: HashMap<String,Vec<(i32,String)>> = HashMap::new();
        for mut words in self.input.iter().map(|x| x.split(' ').peekable()) {
            let bag = format!("{} {}",words.next().unwrap(), words.next().unwrap());
            let mut contain = Vec::<(i32,String)>::new();
            words.next();
            words.next();
            while words.peek().is_some() {
                if words.peek() == Some(&"no") { break; }
                contain.push((
                    words.next().map(|x| x.parse::<i32>().expect("invalid number")).unwrap(),
                    format!("{} {}", words.next().unwrap(), words.next().unwrap()),
                ));
                words.next();
            }
            rules.insert(bag, contain);
        };
        
        self.answer_a(rules.keys().filter(|x| bag_contains_gold(&rules, x) ).count());
        self.answer_b(count_bags(&rules, &String::from("shiny gold")) -1);
        
    }

    pub fn day08(&mut self) {
        enum RunResult {
            Terminate(i32),
            Loop(i32,i32),
        }
        fn run(mut input: Vec<String>) -> RunResult {
            let mut line = 0i32;
            let mut val = 0i32;
            while let Some(instruction) = input.get_mut(line as usize) {
                match (instruction.get(..3),instruction.get(4..).and_then(|x| x.parse::<i32>().ok()))  {
                    (Some("acc"), Some(i)) => { line += 1; val += i; },
                    (Some("jmp"), Some(i)) => { line += i; },
                    (Some("nop"), Some(_)) => { line += 1; },
                    _ => break,
                }
                *instruction = String::from("");
            }
            if line as usize == input.len() {
                RunResult::Terminate(val)
            } else {
                RunResult::Loop(val, line)
            }
        }
        
        self.answer_a(match run(self.input.clone()) {
            RunResult::Loop(val,_) => val,
            _ => -1,
        });

        for (idx, val) in self.input.iter().enumerate() {
            let mut fixed = self.input.clone();
            fixed[idx as usize] = match (val.get(..3),val.get(4..))  {
                (Some("jmp"), Some(i)) => format!("nop {}", i),
                (Some("nop"), Some(i)) => format!("jmp {}", i),
                _ => continue,
            };
            if let RunResult::Terminate(i) = run(fixed) {
                self.answer_b(i);
                break;
            }
        }
    }

    pub fn day09(&mut self) {
        let numbers = self.input
            .iter()
            .map(|x| x.parse::<i64>().expect("parse int error"))
            .collect::<Vec<i64>>();

        let mut invalid = 0;
        'outer1: for i in 25..numbers.len() {
            for num1 in (i-25)..i {
                for num2 in (num1+1)..i {
                    if numbers[num1] + numbers[num2] == numbers[i] {
                        continue 'outer1;
                    }
                }

            }
            invalid = i;
            self.answer_a(numbers[i]);
            break;        
        }

        'outer2: for num1 in (0..invalid).rev() {
            let mut sum = numbers[num1];
            for num2 in (0..num1).rev() {
                sum += numbers[num2];
                match sum.cmp(&numbers[invalid]) {
                    Equal => {
                        let mut min = numbers[num2];
                        let mut max = numbers[num2];
                        for i in num2+1..num1 {
                            if numbers[i] < min { min = numbers[i]; }
                            if numbers[i] > max { max = numbers[i]; }
                        }
                        self.answer_b(min + max);
                        break 'outer2;
                    }
                    Greater => {
                        continue 'outer2;
                    }
                    _ => {}
                }
            }
        };
    }
}