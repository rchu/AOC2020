use std::collections::HashMap;
use anyhow::{Result, Context, anyhow};

use crate::puzzle::Puzzle;
impl Puzzle {
    pub fn day15(&mut self) -> Result<()> {
        let mut history = HashMap::new();
        let mut round: i32 = 0;
        let mut last_number: i32 = 0;
        for (input_round, input_number) in self.input
            .get(0).ok_or_else(|| anyhow!("Cannot read input line"))?
            .split(',').map(|x| x.parse::<i32>().context("Invalid number in input")).collect::<Result<Vec<i32>>>()?
            .iter().enumerate()
        {
            history.insert(*input_number, input_round as i32);
            round += 1;
            last_number = *input_number;
        }
        history.remove(&last_number);
        for round in round..2020 {
            let this_number = match history.get(&last_number) {
                Some(previous_round) => round - previous_round -1,
                None => 0,
            };
            history.insert(last_number, round-1);
            last_number = this_number;
        }
        self.set_answer_a(last_number);
        for round in 2020..30_000_000 {

            let this_number = match history.get(&last_number) {
                Some(previous_round) => round - previous_round -1,
                None => 0,
            };
            history.insert(last_number, round-1);
            last_number = this_number;
        }
        self.set_answer_b(last_number);
        Ok(())
    }
}