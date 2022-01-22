use std::mem;

use crate::Puzzle;
mod day1_;
mod day;

fn between<T: std::cmp::PartialOrd>(value: T, low: T, high: T) -> bool { (low <= value) && (value <= high) }

impl Puzzle {
    pub fn solve(&mut self)  {
        let input = mem::take(&mut self.input);
        let x = match self.day {
            1 => day1_::day01(input),
            2 => day1_::day02(input),
            3 => day1_::day03(input),
            4 => day1_::day04(input),
            5 => day1_::day05(input),
            6 => day1_::day06(input),
            7 => day1_::day07(input),
            8 => day1_::day08(input),
            _ => (None,None),
        };
        self.output_a = x.0;
        self.output_b = x.1;
    }
}