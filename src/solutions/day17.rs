use std::mem;
use anyhow::Result;
use crate::puzzle::Puzzle;

#[derive(Clone)]
struct Space4D { data: Vec<Vec<Vec<Vec<bool>>>> }
impl Space4D {
    fn count_range(&self, axis: [usize; 4], range: [usize; 4]) -> usize {
        self.count_absolute(
            [axis[0]-range[0],   axis[1]-range[1],  axis[2]-range[2],   axis[3]-range[3]  ],
            [axis[0]+range[0]+1, axis[1]+range[1]+1,axis[2]+range[2]+1, axis[3]+range[3]+1],
        )
    }
    fn count_absolute(&self, from: [usize; 4], to: [usize; 4]) -> usize {
        self.data[from[0]..to[0]].iter().map(|vvvb|
                vvvb[from[1]..to[1]].iter().map(|vvb|
                    vvb[from[2]..to[2]].iter().map(|vb|
                        vb[from[3]..to[3]].iter().filter(|b| **b).count()
                    ).sum::<usize>()
                ).sum::<usize>()
            ).sum::<usize>()
    }
    fn iter_over_x(&self, next: &mut Self, xrange: [usize; 2], pos: [usize;3], range: [usize; 3]) {
        let mut neighbours = (
            0, 
            0,
            self.count_range([1, pos[0], pos[1], pos[2]], [0, range[0], range[1], range[2]]),
        );
        for x in xrange[0]..xrange[1] {
            neighbours.0 = neighbours.1;
            neighbours.1 = neighbours.2;
            neighbours.2 = self.count_range([x+1, pos[0], pos[1], pos[2]], [0, range[0], range[1], range[2]]);
            next.data[x][pos[0]][pos[1]][pos[2]] = match (self.data[x][pos[0]][pos[1]][pos[2]], neighbours.0 + neighbours.1 + neighbours.2) {
                (true,3) |
                (true,4) => true,
                (true,_) => false, 
                (false,3) => true,
                (false,_) => false,
            };
        }
    }
}

impl Puzzle {
    pub fn day17(&mut self) -> Result<()> {
        let rounds = 6;
        let size = self.input.len()+2*rounds+4;
        let mut next_3d = Space4D { data: vec![vec![vec![vec![false; 1]; size]; size]; size] };
        let mut next_4d = Space4D { data: vec![vec![vec![vec![false; size]; size]; size]; size] };
        let mut curr_3d = next_3d.clone();
        let mut curr_4d = next_4d.clone();
        for (y, row) in self.input.iter().enumerate() {
            for (x,chr) in row.chars().enumerate() {
                curr_3d.data[x+rounds][y+rounds][size/2][0]      =  chr == '#';
                curr_4d.data[x+rounds][y+rounds][size/2][size/2] =  chr == '#';
            }
        }
        for _ in 0..rounds {
            for y in 1..size-1 {
                for z in 1..size-1 { 
                    for w in 1..size-1 {
                        curr_4d.iter_over_x(&mut next_4d,[1, size-1], [y, z, w], [1, 1, 1]);
                    }
                    curr_3d.iter_over_x(&mut next_3d,[1, size-1], [y, z, 0], [1, 1, 0]);
                }
            }
            mem::swap(&mut curr_3d,&mut next_3d);
            mem::swap(&mut curr_4d,&mut next_4d);
        }
        self.set_answer_a(curr_3d.count_absolute([0,0,0,0],[size, size, size, 1]));
        self.set_answer_b(curr_4d.count_absolute([0,0,0,0],[size, size, size, size]));
        Ok(())
    }
}