use anyhow::Result;
use crate::puzzle::Puzzle;

impl Puzzle {
    pub fn day25(&mut self) -> Result<()> {
        let mut iter = self.input.iter().map(|x| x.parse::<u64>());
        let card_pub = iter.by_ref().next().unwrap()?;
        let door_pub = iter.by_ref().next().unwrap()?;
        let mut remainder = 1;
        for i in 1.. {
            remainder = (remainder * 7) % 20201227; 
            if remainder == card_pub {
                remainder = 1;
                for _ in 0..i { remainder = (remainder * door_pub) % 20201227; }
                break;
            }
            if remainder == door_pub { 
                remainder = 1;
                for _ in 0..i { remainder = (remainder * card_pub) % 20201227; }
                break;
            }
        }
        self.set_answer_a(remainder);
        self.set_answer_b("Sorry for the trouble");
        Ok(())
    }
}
