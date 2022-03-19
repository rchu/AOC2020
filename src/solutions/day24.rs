use std::collections::HashMap;
use std::cmp::Ordering;
use anyhow::{Result, bail};
use crate::puzzle::Puzzle;

// Hex grid coordinates are with offset rows:
//
//         -1,-1   +0,-1   +1,-1  
//     -1,+0   +0,+0   +1,+0   
// -1,+1   +1,+1   +1,+1   
//
impl Puzzle {
    pub fn day24(&mut self) -> Result<()> {

        // Read input: tile-flips 
        let mut tile_flips: Vec<(i32,i32)> = Vec::with_capacity(self.input.len());
        for line in &self.input {
            let mut chrs = line.chars();
            let mut x = 0;
            let mut y = 0;
            while let Some(chr) = chrs.by_ref().next() {
                match chr {
                    'e' => { x += 1; },
                    'w' => { x -= 1; },
                    'n' => match chrs.by_ref().next() {
                        Some('e') => { y -= 1;},
                        Some('w') => { y -= 1; x -= 1; },
                        Some(ch2) => bail!("Unknown direction '{}{}' in line '{}'",chr,ch2,line),
                        None      => bail!("Unknown direction '{}' in line '{}'",chr,line),
                    },
                    's' => match chrs.by_ref().next() {
                        Some('e') => { y += 1; x += 1; },
                        Some('w') => { y += 1; },
                        Some(ch2) => bail!("Unknown direction '{}{}' in line '{}'",chr,ch2,line),
                        None      => bail!("Unknown direction '{}' in line '{}'",chr,line),
                    },
                    chr => bail!("Unknown direction '{}' in line '{}'",chr,line),
                }
            }
            tile_flips.push((x,y));
        }

        // Sort (required for getting black tiles from flips)
        // Sort by  by y, then x (Required for game loop which loops over rows}
        tile_flips.sort_unstable_by(|a,b| match a.1.partial_cmp(&b.1).unwrap() { Ordering::Equal => a.0.partial_cmp(&b.0).unwrap(), x => x });
        // Go through all flips, add all odd occurences
        let mut black_tiles = Vec::with_capacity(tile_flips.len());
        let mut last = (i32::MAX, i32::MAX);
        let mut flip = false;
        for res in tile_flips {
            if res == last {
                flip = !flip;
            } else {
                if flip { black_tiles.push(last); }
                last = res;
                flip = true;
            }
        }
        if flip { black_tiles.push(last); }
        self.set_answer_a(black_tiles.len());

        // Game loop
        let mut round = 0;
        while round < 100 {
            let mut new_black_tiles = Vec::with_capacity(black_tiles.capacity());
            let mut prev_row = HashMap::with_capacity(32);
            let mut curr_row = HashMap::with_capacity(32);
            let mut next_row = HashMap::with_capacity(32);
            let mut curr_y = black_tiles.first().or(Some(&(0,0))).unwrap().1;
            
            for (bl_x, bl_y) in black_tiles {
                while bl_y > curr_y {
                    // white: n=2 black: n=6+1 or n=6+2
                    for (x,count) in prev_row {
                        if count == 2 || count == 7 || count == 8 { new_black_tiles.push((x,curr_y-1)); }
                    }
                    curr_y += 1;
                    prev_row = curr_row;
                    curr_row = next_row;
                    next_row = HashMap::with_capacity(32);
                }
                // add one black neighbour count to all neighbours (and 6 to self)
                *prev_row.entry(bl_x-1).or_insert(0) += 1;
                *prev_row.entry(bl_x  ).or_insert(0) += 1;
                *curr_row.entry(bl_x-1).or_insert(0) += 1;
                *curr_row.entry(bl_x  ).or_insert(0) += 6;
                *curr_row.entry(bl_x+1).or_insert(0) += 1;
                *next_row.entry(bl_x  ).or_insert(0) += 1;
                *next_row.entry(bl_x+1).or_insert(0) += 1;
            }
            // process remaining rows
            for (y,row) in [(curr_y-1, prev_row), (curr_y, curr_row), (curr_y+1, next_row)] {
                for (x,count) in row {
                    if count == 2 || count == 7 || count == 8 {
                        new_black_tiles.push((x,y));
                    }
                }
            }
            // new result, next round
            black_tiles = new_black_tiles;
            round += 1;

        }
        self.set_answer_b(black_tiles.len());
        Ok(())
    }
}