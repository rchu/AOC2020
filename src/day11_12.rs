use std::{usize};
use anyhow::{Result, Context, bail};
use crate::puzzle::Puzzle;

#[derive(Clone, Copy, PartialEq)]
enum Seat { Floor, Empty, Occupied }
struct Seats {
    state: Vec<Seat>,
    width: usize,
    height: usize,
    changed: usize,
}
impl Seats {
    pub fn from_input(input: &[String]) -> Result<Self> {
        if input.is_empty() || input[0].is_empty() {
            bail!("Input has width and/or height of zero");
        }
        let mut state = vec![];//Vec::with_capacity();
        for line in input.iter() {
            for chr in line.chars() {
                match chr {
                    'L' => state.push(Seat::Empty),
                    '.' => state.push(Seat::Floor),
                    '#' => state.push(Seat::Occupied),
                    c =>  bail!("Invalid character '{}' in input",c),
                };
            }
        }
        Ok(Self {
            state,
            width: input[0].chars().count(),
            height: input.len(),
            changed: 0,
        })
    }

    fn get_state(&self, x: i32,y: i32) -> Option<Seat> {
        if x < 0 || y < 0 { return None; }  
        let ux = x as usize;
        let uy = y as usize;
        if ux >= self.width || uy >= self.height {
            None
        } else {
            Some(self.state[(self.width * uy + ux) as usize])
        }
    }
    fn surrounding_occupied_seats_1(&self, x: usize ,y: usize) -> i8 {
        let mut surrounding_occupied_seats = 0;
        for nx in if x==0 {0} else {x-1}..self.width.min(x+2) {
        for ny in if y==0 {0} else {y-1}..self.height.min(y+2) {
            if (x!=nx || y!=ny) 
            && self.state[nx + self.width*ny] == Seat::Occupied {
                surrounding_occupied_seats += 1;
        }}}
        surrounding_occupied_seats
    }
    fn surrounding_occupied_seats_2(&self, x: usize ,y: usize) -> i8 {
        let mut surrounding_occupied_seats = 0;
        for dx in -1..=1 {
        for dy in -1..=1 {
        if 0!=dx || 0!=dy {
            for dist in 1.. {
                match self.get_state((x as i32)+dist*dx, (y as i32)+dist*dy) {
                    Some(Seat::Occupied) => surrounding_occupied_seats += 1,
                    Some(Seat::Floor) => continue,
                    _ => {},
                }
                break;
        }}}}
        surrounding_occupied_seats              
    }

    pub fn next_state(&mut self, emp_2_occ: i8, occ_2_emp: i8, part_1_or_2: i8) -> &Self {
        let mut next = Vec::with_capacity(self.state.capacity());
        self.changed = 0;
        for y in 0..self.height {
        for x in 0..self.width {
            let surrounding_occupied_seats =  if part_1_or_2 == 1 {
                self.surrounding_occupied_seats_1(x, y)
            } else {
                self.surrounding_occupied_seats_2(x, y)
            };
            next.push(
                if self.state[x + self.width*y] == Seat::Empty
                && surrounding_occupied_seats == emp_2_occ {
                    self.changed += 1;
                    Seat::Occupied
                } else if self.state[x + self.width*y] == Seat::Occupied
                && surrounding_occupied_seats >= occ_2_emp {
                    self.changed += 1;
                    Seat::Empty
                } else {
                    self.state[x + self.width*y]
                }
            );
        }}
        self.state = next;
        self
    }
}
impl Puzzle {
    pub fn day11(&mut self) -> Result<()> {
        let mut seats = Seats::from_input(&self.input).context("Cannot create seats map")?;
        self.set_answer_a(loop {
            if seats.next_state(0, 4, 1).changed == 0 {
                break seats.state.iter().filter(|&s| s == &Seat::Occupied).count();
            }
        });
        let mut seats = Seats::from_input(&self.input).context("Cannot create seats map")?;
        self.set_answer_b(loop {
            if seats.next_state(0, 5, 2).changed == 0 {
                break seats.state.iter().filter(|&s| s == &Seat::Occupied).count();
            }
        });
        Ok(())
    }

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
