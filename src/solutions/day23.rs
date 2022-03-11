use anyhow::{Result, bail, anyhow};
use crate::puzzle::Puzzle;


const A_ROUNDS: usize = 100;
const A_CUPS:   usize = 9;
const B_ROUNDS: usize = 10_000_000;
const B_CUPS:  usize = 1_000_000;

fn make_cups_a(cups: [usize; 9]) -> [usize; A_CUPS+1] {
    let mut result = [0;A_CUPS+1];
    // change input with cups param
    let mut cur = 0;
    for nxt in cups {
        result[cur] = nxt;
        cur = nxt;
    }
    result[cur] = if A_CUPS > 9 {10} else {cups[0]};
    result[0] = cups[0];
    // Return 
    result
}
fn make_cups_b(cups: [usize; 9]) -> [usize; B_CUPS+1] {
    let mut result = [0; B_CUPS+1];
    // change input with cups param
    let mut cur = 0;
    for nxt in cups {
        result[cur] = nxt;
        cur = nxt;
    }
    result[cur] = if B_CUPS > 9 {10} else {cups[0]};
    result[0] = cups[0];
    // Create input in order 1..len
    for (idx, item) in result.iter_mut().enumerate().skip(A_CUPS+1) { *item = idx+1; };
    result[B_CUPS] = cups[0];
    // Return
    result
}

fn answer_a(cups: &[usize]) -> String {
    let mut result = String::with_capacity(A_CUPS-1);
    let mut idx = cups[1];
    for _ in 1..A_CUPS {
        result.push_str(&idx.to_string());
        idx = cups[idx];
    }
    result
}
fn answer_b(cups: &[usize]) -> usize {
    cups[1] * cups[cups[1]]
}

fn cups_game(cups: &mut [usize], len: usize, rounds: usize) {
    for _round in 1..=rounds {
        let pick1 = cups[cups[0]];
        let pick2 = cups[pick1];
        let pick3 = cups[pick2];
        let pick_after = cups[pick3];
        let mut dst = match cups[0] {1 => len, x => x - 1 };
        while dst == pick1 || dst == pick2 || dst == pick3 { dst = match dst {1 => len, x => x - 1 } };
        let dst_after = cups[dst];

        cups[cups[0]] = pick_after;
        cups[0] = pick_after;
        cups[dst] = pick1;
        cups[pick3] = dst_after;
    }
}

impl Puzzle {
    pub fn day23(&mut self) -> Result<()> {
        // Read input, do some error checking
        let mut input: [usize; 9] = [0;9];
        let line = self.input.get(0).ok_or_else(|| anyhow!("Cannot read input line"))?;
        if line.chars().count() != 9 { bail!("Input line must be 9 chars long")};
        for (i,chr) in line.chars().enumerate() {
            if let Some(d) = chr.to_digit(10) {
                if d > 0 && d < 10 { 
                    input[i] = d as usize;
                } else {
                    bail!("Digit not in range 1..9");
                }
            } else {
                bail!("Invalid digit '{}' in input",chr);
            }
        }
        // Game A
        let mut game = make_cups_a(input);
        cups_game(&mut game, A_CUPS, A_ROUNDS);
        self.set_answer_a(answer_a(&game));
        // Game B
        let mut game = make_cups_b(input);
        cups_game(&mut game, B_CUPS, B_ROUNDS);
        self.set_answer_b(answer_b(&game));
 
        Ok(())
    }
}