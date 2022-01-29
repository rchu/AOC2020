use crate::puzzle::Puzzle;

impl Puzzle {
    pub fn day01(&mut self) {
        let numbers: Vec<i32> = self.input
            .iter()
            .map(|x|{x.parse::<i32>().expect("cannot convert input into i32")})
            .collect();
        
        'outer1: for i1 in 0..numbers.len() {
            for i2 in (i1+1)..numbers.len() {
                if numbers[i1] + numbers[i2] == 2020 {
                    self.set_answer_a(numbers[i1] * numbers[i2]);
                    break 'outer1;
                }
            }
        }

        'outer2: for i1 in 0..(numbers.len()) {
            for i2 in (i1+1)..(numbers.len()) {
                for i3 in (i2+1)..numbers.len() {
                    if numbers[i1] + numbers[i2] + numbers[i3] == 2020 {
                        self.set_answer_b(numbers[i1] * numbers[i2] * numbers[i3]);
                        break 'outer2;
                    } 
                }
            }
        }
    }
}