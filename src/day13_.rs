use std::{usize};
use anyhow::{Result, Context, anyhow, Error};
use crate::puzzle::Puzzle;
impl Puzzle {
    pub fn day13a(&mut self) -> Result<()> {
        let time = self.input
            .get(0).ok_or(anyhow!("Cannot read first line"))?
            .parse::<i32>().context("Cannot parse timestamp")?;
        let mut busses = self.input
            .get(1).ok_or(anyhow!("Cannot read second line"))?
            .split(',').filter(|&x| x != "x")
            .map(|x| 
                x.parse::<i32>().context("Cannot parse bus ID").map(|m|
                    (m-(time % m), (m-(time % m))*m)
                )
            )
            .collect::<Result<Vec<(i32,i32)>,Error>>()?;
        busses
            .sort_by(|x,y| x.0.cmp(&y.0));

        self.set_answer_a(busses.get(0).ok_or_else(|| anyhow!("No busses found"))?.1);
        Ok(())
    }
    pub fn day13b(&mut self) -> Result<()> {
        let busses = self.input
            .get(1).ok_or(anyhow!("Cannot read second line"))?
            .split(',').enumerate().filter(|&x| x.1 != "x")
            .map(|x| 
                x.1.parse::<usize>().context("Cannot parse bus ID").map(|x1| (x1, x.0))
            )
            .collect::<Result<Vec<(usize,usize)>,Error>>()?;

        let mut interval = 1;
        let mut prod_bus_time = 1;
        for bus in busses.iter() {
            let mut new_interval = interval;
            interval = loop {
                if (new_interval + bus.1) % bus.0 == 0 { break new_interval; }
                new_interval += prod_bus_time;
            };
            prod_bus_time *= bus.0;
        }
        self.set_answer_b(interval);  
        Ok(())
    }
}