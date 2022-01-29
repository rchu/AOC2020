use std::cmp::Ordering::{Greater, Equal};
use anyhow::Result;
use crate::puzzle::Puzzle;
impl Puzzle {
    pub fn day09(&mut self) -> Result<()> {
        let numbers = self.get_input_as::<i64>()?;

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
            self.set_answer_a(numbers[i]);
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
                        #[allow(clippy::needless_range_loop)]
                        for i in num2+1..num1 { 
                            if numbers[i] < min { min = numbers[i]; }
                            if numbers[i] > max { max = numbers[i]; }
                        }
                        self.set_answer_b(min + max);
                        break 'outer2;
                    }
                    Greater => {
                        continue 'outer2;
                    }
                    _ => {}
                }
            }
        };
        Ok(())
    }
}