use std::usize;
use anyhow::{Result, Context, bail};
use crate::puzzle::Puzzle;

impl Puzzle {
    pub fn day12a(&mut self) -> Result<()> {
        const DIRS: [[i32; 2]; 4] = [[1,0], [0,-1], [-1,0], [0,1]];
        let mut pos = [0, 0];
        let mut dir = 0;
        for mut line in self.input.iter().map(|s| s.chars()) {
            match (
                line.next(),
                line.take(usize::MAX).collect::<String>()
                    .parse::<i32>().context("Error parsing instruction value")?
            )  {
                (Some('N'),delta) => pos[1] += delta,
                (Some('E'),delta) => pos[0] += delta,
                (Some('S'),delta) => pos[1] -= delta,
                (Some('W'),delta) => pos[0] -= delta,
                (Some('L'),delta) => {
                    dir = (dir - (delta/90) + 4) % 4
                },
                (Some('R'),delta) => { dir = (dir + (delta/90)) % 4 },
                (Some('F'),delta) => {
                    pos[0] += DIRS[dir as usize][0] * delta;
                    pos[1] += DIRS[dir as usize][1] * delta;
                },
                (Some(c),delta) => bail!("Invalid instruction: {}{}",c,delta),
                (None,_) => bail!("Invalid instruction: empty line"),

            }
        }
        self.set_answer_a(pos[0].abs() + pos[1].abs());
        Ok(())
    }

    pub fn day12b(&mut self) -> Result<()> {
        let mut pos = [0, 0];
        let mut waypoint = [10,1];
        for mut line in self.input.iter().map(|s| s.chars()) {
            match (
                line.next(),
                line.take(usize::MAX).collect::<String>()
                    .parse::<i32>().context("Error parsing instruction value")?
            )  {
                (Some('N'),delta) => waypoint[1] += delta,
                (Some('E'),delta) => waypoint[0] += delta,
                (Some('S'),delta) => waypoint[1] -= delta,
                (Some('W'),delta) => waypoint[0] -= delta,
                (Some('L'),delta) => { for _ in 0..(delta/90 + 4) % 4 { waypoint = [-waypoint[1], waypoint[0]]; }},
                (Some('R'),delta) => { for _ in 0..(delta/90 + 4) % 4 { waypoint = [waypoint[1], -waypoint[0]]; }},
                (Some('F'),delta) => {
                    pos[0] += waypoint[0] * delta;
                    pos[1] += waypoint[1] * delta;
                },
                (Some(c),delta) => bail!("Invalid instruction: {}{}",c,delta),
                (None,_) => bail!("Invalid instruction: empty line"),
            }
        }
        self.set_answer_b(pos[0].abs() + pos[1].abs());
        Ok(())
    }
}
