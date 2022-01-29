use std::collections::HashMap;
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
        
        self.set_answer_a(rules.keys().filter(|x| bag_contains_gold(&rules, x) ).count());
        self.set_answer_b(count_bags(&rules, &String::from("shiny gold")) -1);
        
    }
}