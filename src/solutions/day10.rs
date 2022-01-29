use anyhow::{Result, Context};
use crate::puzzle::Puzzle;
impl Puzzle {
    pub fn day10(&mut self) -> Result<()> {
        let mut adapters = self.get_input_as::<i32>().with_context(|| "Cannot parse input")?;
        adapters.push(0);
        adapters.sort_unstable();
        adapters.push(*adapters.last().unwrap()+3);
        
        let mut diff = [0,0,0,0];
        for i in 0..adapters.len()-1 {
            diff[(adapters[i+1] - adapters[i]) as usize] += 1;
        }
        self.set_answer_a(diff[1] * diff[3]);

        let mut paths = vec![1i64; adapters.len()];
        for idx in (0..adapters.len()-1).rev() {
            paths[idx] = (idx+1..adapters.len()).take(3)
                .filter( |&target| adapters[target] - adapters[idx] <= 3)
                .map( |target| paths[target])
                .sum();
        }
        self.set_answer_b(paths[0]);
        Ok(())
    }
}