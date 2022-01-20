use crate::Puzzle;

mod day1_;
mod day;

impl Puzzle {
    pub fn solve(&mut self)  {
        self.output = match self.day {
            1 => day1_::day01(&self.input),
            2 => day1_::day02(&self.input),
            3 => day1_::day03(&self.input),
            4 => day1_::day04(&self.input),
            5 => day1_::day05(&self.input),
            _ => None,
        }
    }
}