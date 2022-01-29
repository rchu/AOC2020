use std::{collections::HashMap, slice::Iter};
use anyhow::{Result, Context, anyhow, bail};

use crate::puzzle::Puzzle;
impl Puzzle {
    pub fn day14(&mut self) -> Result<()> {
        let mut memory: HashMap<u64,u64> = HashMap::new();
        let mut mask_and: u64 = u64::MAX;
        let mut mask_or: u64 = 0;
        for line in self.input.iter() {
            match line.get(0..4) {
                Some("mask") => {
                    mask_and = 0u64;
                    mask_or  = 0u64;
                    for chr in line.get(7..).ok_or_else(||anyhow!("Cannot read mask in '{}'", line))?.chars() {
                        mask_and <<= 1;
                        mask_or <<= 1;
                        match chr {
                            'X' => mask_and |= 1,
                            '0' => {},
                            '1' => mask_or |= 1,
                            chr => bail!("Invalid character '{}' in mask", chr),
                        }
                    }
                },
                Some("mem[") => {
                    let mem = line.get(4..).ok_or_else(|| anyhow!("Cannot read mem instruction in line '{}'", line))?
                        .split("] = ").map(|x| x.parse::<u64>().context("Cannot parse mem"))
                        .collect::<Result<Vec<u64>>>()?;
                    memory.insert(mem[0], mem[1] & mask_and | mask_or);
                },
                _ => bail!("Invalid instruction '{}'", line),
            }
        }
        self.set_answer_a(memory.values().sum::<u64>());

        fn floating_addr(addr: u64, float: &mut Iter<u64>) -> Vec<u64> {
            match float.next() {
                None => vec![addr],
                Some(f) => {
                    let mut result = floating_addr(addr, float);
                    result.append(&mut result.iter().map(|x| x ^ f).collect::<Vec<u64>>());
                   result
        }}}
        memory = HashMap::new();
        mask_or = 0;
        let mut mask_float: Vec<u64> = Vec::new();
        for line in self.input.iter() {
            match line.get(0..4) {
                Some("mask") => {
                    mask_or = 0;
                    mask_float.clear();
                    for chr in line.get(7..).ok_or_else(|| anyhow!("Cannot read mask in '{}'", line))?
                        .chars().enumerate() {
                        match chr {
                            (_, '0') => {},
                            (n, '1') => mask_or |= 2u64.pow(35-(n as u32)),
                            (n, 'X') => mask_float.push(2u64.pow(35-(n as u32))),
                            (n, chr) => bail!("Invalid character #{} '{}' in mask", n, chr),
                        }
                    }
                },
                Some("mem[") => {
                    let mem = line.get(4..).ok_or_else(|| anyhow!("Cannot read mem instruction in line '{}'", line))?
                        .split("] = ").map(|x| x.parse::<u64>().context("Cannot parse mem"))
                        .collect::<Result<Vec<u64>>>()?;
                    for addr in floating_addr(mem[0] | mask_or, &mut mask_float.iter()) {
                        memory.insert(addr, mem[1]);
                    }
                },
                _ => bail!("Invalid instruction '{}'", line),
            }
        }
        self.set_answer_b(memory.values().sum::<u64>());
        Ok(())
    }
}