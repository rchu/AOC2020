use std::collections::HashMap;
use anyhow::Result;
use anyhow::anyhow;
use array_tool::vec::Intersect;
use array_tool::vec::Join;
use crate::puzzle::Puzzle;

impl Puzzle {
    pub fn day21(&mut self) -> Result<()> {

        let mut allergens: HashMap<&str, Vec<&str>> = HashMap::new();
        let mut ingredients: HashMap<&str,usize> = HashMap::new();
        let input = self.input.clone();
        for line in &input {
            let (rule_ingredients, rule_allergens) = line.split_once(" (contains ").ok_or_else(|| anyhow!("no contains"))?;
            let rule_ingredients = rule_ingredients.split_whitespace().collect::<Vec<&str>>();
            for &ingr in &rule_ingredients { 
                ingredients.entry(ingr)
                    .and_modify(|x| *x += 1)
                    .or_insert(1);
            }
            for alrg in rule_allergens.split(", ").map(|x| x.trim_matches(')')) {
                allergens.entry(alrg)
                    .and_modify(|ingr| *ingr = ingr.intersect(rule_ingredients.clone()))
                    .or_insert_with(|| rule_ingredients.clone());
            }
        }

        let mut allergens_one: Vec<(&str, &str)> = allergens.iter()
            .filter_map(|(&alrg, ingr)|
                if ingr.len() == 1 {Some((alrg,ingr[0]))} else {None}
            ).collect();
        while let Some((alrg_one, ingr_one)) = allergens_one.pop() {
            for (&alrg, ingr) in allergens.iter_mut() {
                if alrg_one == alrg || ingr.len() == 1 { continue}
                ingr.retain(|i| !ingr_one.contains(i));
                if ingr.len() == 1 { allergens_one.push((alrg, ingr[0])); }
            }
        }

        let ingredients_allergents = allergens.iter().flat_map(|(_, x)| x.clone() ).collect::<Vec<&str>>();
        self.set_answer_a(
            ingredients.iter()
                .map(|(ingr,count)| if ingredients_allergents.contains(ingr) {0} else {*count})
                .sum::<usize>()
        );
        let mut allergens = Vec::from_iter(allergens.iter().map(|(&alrg, ingr)| (alrg, ingr[0])));
        allergens.sort_unstable_by(|a,b| a.0.partial_cmp(b.0).unwrap());
        self.set_answer_b(
            allergens.iter()
                .map(|&x| x.1)
                .collect::<Vec<&str>>()
                .join(",")
        );
            
        Ok(())
    }
}