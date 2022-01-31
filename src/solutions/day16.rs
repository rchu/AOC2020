use std::collections::HashMap;
use std::num::ParseIntError;
use anyhow::{Result,bail, anyhow};
use array_tool::vec::Shift;

use crate::puzzle::Puzzle;
impl Puzzle {
    #[allow(clippy::while_let_on_iterator)] // iteration is broken up and continued later
    pub fn day16(&mut self) -> Result<()> {
        let mut input = self.input.iter();
        
        let mut rule_name: HashMap<i32, String> = HashMap::new();
        let mut rule_range: HashMap<i32,(i32, i32, i32,i32)> = HashMap::new();
        let mut rule_count = 0;
        while let Some(line) = input.next() {
            if line.is_empty() { break; }
            let (name, ranges) = line.split_once(": ").ok_or_else(|| anyhow!("No ': ' found in ticket rules"))?;
            let ranges = ranges.split(" or ")
            .flat_map(|x| x.split('-'))
            .map(|x| x.parse::<i32>())
            .collect::<Result<Vec<i32>,ParseIntError>>()?;
            rule_name.insert(rule_count, String::from(name));
            rule_range.insert(rule_count,(ranges[0],ranges[1], ranges[2], ranges[3]));
            rule_count += 1;
        }
        
        if input.next() != Some(&String::from("your ticket:")) { bail!("Expected the 'your ticket' section"); } 
        let my_ticket = input.next().ok_or_else(|| anyhow!("Cannot read your ticket"))?
            .split(',').map(|x| x.parse::<i32>())
            .collect::<Result<Vec<i32>, ParseIntError>>()?;
        input.next();

        if input.next() != Some(&String::from("nearby tickets:")) { bail!("Expeced the 'nearby tickets' section"); } 
        let mut sum_invalid_nrs = 0;
        let mut tickets: Vec<Vec<i32>> = Vec::new();
        while let Some(line) = input.next() {
            let mut valid = true;
            let ticket = line.split(',').map(|x| x.parse::<i32>()).collect::<Result<Vec<i32>, ParseIntError>>()?;
            for nr in ticket.iter() {
                if rule_range.iter().all(|(_, x)| (x.0 > *nr || *nr > x.1) && (x.2 > *nr || *nr > x.3)) {
                    sum_invalid_nrs += nr;
                    valid = false;
                }
            }
            if valid {
                tickets.push(ticket);
            }
        }
        
        let mut nr_valid_for: HashMap<usize, Vec<i32>> = HashMap::with_capacity(my_ticket.len());
        let mut nr_todo: Vec<usize> = Vec::with_capacity(my_ticket.len());
        let mut result: Vec<(usize,i32)> = Vec::with_capacity(my_ticket.len());
        for nth_number in 0..my_ticket.len() {
            nr_todo.push(nth_number);
            nr_valid_for.insert(
                nth_number,
                rule_range.iter().filter_map(|(rule, range)|
                    if tickets.iter().all(|nrs| {
                        let nr = *nrs.get(nth_number).unwrap();
                        (range.0 <= nr && nr <= range.1) || (range.2 <= nr && nr <= range.3)
                    }) {
                        Some(*rule)
                    } else {
                        None
                    }
                ).collect()
            );
        }
        while !nr_todo.is_empty() {
            nr_todo.sort_unstable_by(|x,y| nr_valid_for[x].len().cmp(&nr_valid_for[y].len()));
            let nr = nr_todo.shift().unwrap();
            if nr_valid_for[&nr].len() > 1 { bail!("Implementation onlyt works when there is a column witb only one option"); }
            let field =  nr_valid_for.remove(&nr).unwrap().pop().unwrap();
            nr_valid_for.iter_mut().for_each(|(_,fields)| fields.retain(|x| x != &field));
            result.push((nr,field));

        }
        self.set_answer_a(sum_invalid_nrs);
        self.set_answer_b(result.iter()
            .filter_map(|(nr, field)|
                if rule_name.get(field).unwrap().get(0..9) == Some("departure") {
                    Some(my_ticket[*nr] as i64)
                } else {
                    None
                }
            ).product::<i64>());
        Ok(())
    }
}