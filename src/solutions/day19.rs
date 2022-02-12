use std::vec;
use std::num::ParseIntError;
use std::collections::HashMap;
use anyhow::{Result, anyhow, bail, Context};
use crate::puzzle::Puzzle;

#[derive(Clone, Default)]
struct MessageRules {
    max_rule_idx: u16,
    term: HashMap<u16, Vec<char>>,
    one:  HashMap<u16, Vec<u16>>,
    two:  HashMap<u16, Vec<(u16, u16)>>,
}
impl MessageRules {
    fn from_input(lines: &[String]) -> Result<Self> {
        let mut result = Self::default();
        for line in lines {
            let (from, tail) = line.split_once(": ")
                .ok_or_else(|| anyhow!("no ':' (input line '{}')", line))?;
            let from: u16 = from.parse()
                .with_context(|| format!("cannot parse rule number '{}' (input line '{}')",from, line))?;
            
            if tail.is_empty() {
                bail!("empty rule body (input line '{}')", line);
            } else if tail.contains('"') {
                result.term.insert(from, vec![
                    tail.chars().into_iter()
                    .nth(1)
                    .ok_or_else(|| anyhow!("cannot parse terminal rule number '{}' (input line '{}')",from, line))?
                ]);
            } else {
                let parts = tail.split(" | ").map(|or| or
                        .split_whitespace().map(|x| x
                        .parse()
                    ).collect()
                    ).collect::<Result<Vec<Vec<u16>>, ParseIntError>>()
                    .map_err(|err| anyhow!("{} (input line '{}')",err, line))?;
                if parts.is_empty() {
                    bail!("empty rule body (input line '{}')", line);
                } else {
                    let mut res1 = Vec::new();
                    let mut res2 = Vec::new();
                    for part in parts {
                        if part.len() == 1 {
                            res1.push(part[0]); 
                        } else if part.len() == 2 {
                            res2.push((part[0], part[1]));
                        } else {
                            bail!("rule with {} numbers not implemented (input line '{}')",part.len(), line);
                        }    
                    }
                    if !res1.is_empty() { result.one.insert(from, res1); }
                    if !res2.is_empty() { result.two.insert(from, res2); }
                }
                ;
                result.max_rule_idx = result.max_rule_idx.max(from);  
            }
        }
        Ok(result)
    }
    /// Convert rules to Chompsky Normal Form (only step needed is UNIT)
    fn cnf(mut self) -> Self {
        let mut rules_1: Vec<(u16, Vec<u16>)> = self.one.into_iter().collect();
        while let Some((rem_from, rem_tos)) = rules_1.pop() {
            let mut new_rules_t = Vec::new();
            let mut new_rules_1 = Vec::new();
            let mut new_rules_2 = Vec::new();
            for rem_to in rem_tos {
                for (from, to) in &rules_1 {
                    if rem_to == *from { new_rules_1.push((rem_from, to.to_owned())); }}   
                for (from, to) in &self.two {
                    if rem_to == *from { new_rules_2.push((rem_from, to.to_owned())); }}    
                for (from, to) in &self.term {
                    if rem_to == *from { new_rules_t.push((rem_from, to.to_owned())); }}    
            }
            for (from, to) in new_rules_1 {
                rules_1.push((from, to));
            }
            for (from, mut to) in new_rules_2 {
                if self.two.contains_key(&from) {
                    let mut existing = self.two.remove(&from).unwrap();
                    to.append(&mut existing);
                }
                self.two.insert(from, to);
            }
            for (from, mut to) in new_rules_t {
                
                if self.term.contains_key(&from) {
                    let mut existing = self.term.remove(&from).unwrap();
                    to.append(&mut existing);
                }
                
                self.term.insert(from, to);
            }        
        }
        self.one = HashMap::new();
        self
    }
    /// tries left-first to expand from the starting rule 
    fn valid_message(&self, goal: &[char], msg: &[u16]) -> Result<bool> {
        let mut idx = 0;
        // Search for leftmost expandable number
        while let Some(number) = msg.get(idx) {
            if idx >= goal.len() { return Ok(false); }
            if let Some(term) = self.term.get(number) {
                if !term.contains(&goal[idx]) {return Ok(false); }
            } 
            else { break; } 
            idx += 1;
        }
        // Nothing to expand
        if idx == msg.len() { return Ok(idx == goal.len()); }
        // Try with all replacements
        if let Some(replacements) = self.two.get(msg.get(idx).unwrap()) {
            for replacement in replacements {
                let mut new = msg.get(..idx).ok_or_else(|| anyhow!("cannot get head slice ..{} of msg {:?}",idx,msg))?.to_vec();
                new.push(replacement.0);
                new.push(replacement.1);
                new.append(&mut msg.get(idx+1..).ok_or_else(|| anyhow!("cannot get tail slice {}.. of msg {:?}",idx+1,msg))?.to_vec());
                if let Ok(true) = self.valid_message(goal, &new) { return Ok(true); }    
            }
        }
        // No replacement worked
        Ok(false)
    }
    #[allow(dead_code)]
    /// CYK algorithm works like a charm but is very slow
    fn valid_message_cyk(rules: &MessageRules, message: &str) -> Result<bool> {
        let mut p: Vec<(u16,usize, usize)> = Vec::with_capacity(2048);
        for (pos,chr) in message.chars().enumerate() {
            for (from, to) in &rules.term {
                if to.contains(&chr) {
                    p.push((*from, pos, 1));
        }}}
        
        for len in 2..message.len()+1 {
            for start in 0..message.len() - len + 1{
                for split in 1..len {
                    for (from, rule) in &rules.two {
                        for (to1, to2) in rule {
                            if p.contains(&(*to1, start, split)) && p.contains(&(*to2, start+split, len-split)) {
                                p.push((*from, start, len));
        }}}}}}
        Ok(p.contains(&(0,0,message.len())))
    }
}

impl Puzzle {
    
    pub fn day19(&mut self) -> Result<()> {
        // Read input
        let mut input = self.input.split(|line| line.is_empty());
        let mut rules = input.next().ok_or_else(|| anyhow!("no rules found"))?.to_vec();
        let messages = input.next().ok_or_else(|| anyhow!("no messages found"))?.to_vec();

        // Part 1
        let rules_part1 = MessageRules::from_input(&rules)?.cnf();
        let mut valid: i32 = 0;
        for message in &messages {
            if rules_part1.valid_message(&message.chars().collect::<Vec<char>>(), &[0])? {
                valid += 1; }}
        self.set_answer_a(valid);

        // Modify input rules for part 2
        rules.retain(|line| !line.starts_with("8:") || !line.starts_with("11:"));
        rules.push("8: 42 | 42 8".to_string());
        rules.push("11: 42 31 | 42 1000".to_string());
        rules.push("1000: 11 31".to_string());

        let rules_part2= MessageRules::from_input(&rules)?.cnf();
        valid = 0;
        for message in messages {
            if rules_part2.valid_message(&message.chars().collect::<Vec<char>>(), &[0])? {
                valid += 1; }}
        self.set_answer_b(valid);
        Ok(())
    }
}
