#[allow(unused_imports)]
use core::num;
use std::collections::HashMap;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;
use crate::puzzle::Puzzle;
use anyhow::bail;
use anyhow::Result;
use anyhow::anyhow;

const TILE_SIZE: usize = 10;
const TILE_HALF: usize = 5;
const TILE_SIZE2: usize = TILE_SIZE - 2;
const VERBOSE: bool = false;
const VERBOSE_SLEEP: u64 = 0;
const VERBOSE_SLEEP2: u64 = 1;

fn right(x: usize) { if x != 0 { print!("\x1b[{}C", x); }; }
fn up   (y: usize) { if y != 0 { print!("\x1b[{}A", y); }; }
fn down (y: usize) { if y != 0 { print!("\x1b[{}B", y); }; }
fn left (x: usize) { if x != 0 { print!("\x1b[{}D", x); }; }
fn color(odd_even: bool) {
    if odd_even {print!("\x1b[38;5;124;48;5;000m")} else {print!("\x1b[38;5;124;48;5;244m")}
}
fn print_char(top: bool, top_hilight: bool, bottom: bool, bottom_hilight: bool, odd_even: bool) {
    match (top,top_hilight, bottom,bottom_hilight, odd_even) {
        // Top & Bottom
        (true ,true ,  true ,false,    _  ) => print!("\x1b[38;5;124;48;5;228m▄"),
        (true ,false,  true ,true ,    _  ) => print!("\x1b[38;5;228;48;5;124m▄"),
        (true ,true ,  true ,true ,    _  ) => print!("\x1b[48;5;228m "),
        (true ,false,  true ,false,    _  ) => print!("\x1b[48;5;124m "),
        // Top
        (true ,true ,  false,  _  ,  true ) => print!("\x1b[38;5;000;48;5;228m▄"),
        (true ,false,  false,  _  ,  true ) => print!("\x1b[38;5;000;48;5;124m▄"),
        (true ,true ,  false,  _  ,  false) => print!("\x1b[38;5;244;48;5;228m▄"),
        (true ,false,  false,  _  ,  false) => print!("\x1b[38;5;244;48;5;124m▄"),
        // Bottom
        (false,  _  ,  true ,true ,  true ) => print!("\x1b[38;5;228;48;5;000m▄"),
        (false,  _  ,  true ,false,  true ) => print!("\x1b[38;5;124;48;5;000m▄"),
        (false,  _  ,  true ,true ,  false) => print!("\x1b[38;5;228;48;5;244m▄"),
        (false,  _  ,  true ,false,  false) => print!("\x1b[38;5;124;48;5;244m▄"),
        // None
        (false,  _  ,  false,  _  ,  true ) => print!("\x1b[48;5;000m "),
        (false,  _  ,  false,  _  ,  false) => print!("\x1b[48;5;244m "),
    }
}

type Shape = i32;
#[derive(Default,Copy,Clone)]
struct Side {
    shape: Shape,
    rev: Shape,
}
impl Side {
    fn from_chars(char_iter: impl Iterator<Item = char>) -> Self {
        let mut shape = 0;
        let mut rev = 0;
        for chr in char_iter {
            shape <<= 1; 
            rev >>= 1;
            if chr == '#' { shape |= 1; rev |= 0b10_0000_0000;};
        } 
        Self { shape, rev }
    }
    fn get(&self, reversed: bool) -> Self {
        if reversed {
            Self { shape: self.rev, rev: self.shape }
        } else {
            *self
        }
    }
}

type TileID = i32;
#[derive(Default,Clone)]
struct Tile {
    id: TileID,
    rows: [[bool; TILE_SIZE];TILE_SIZE],
    sides: [Side; 4],
} 
impl Tile {
    fn from_input(input: &[String]) -> Result<Self> {
        if input.len() != TILE_SIZE + 1 { bail!("expected input lenght {}, not {}", TILE_SIZE+1, input.len())}
        if !input[0].starts_with("Tile ") { bail!("expected 'Tile <num>' (on line '{}')", input[0]); }
        let id = input[0][5..input[0].len()-1].parse().map_err(|err| anyhow!("{} (on line '{}')", err, input[0]))?;
        let mut rows = [[false; TILE_SIZE];TILE_SIZE];
        for (y,row) in input[1..].iter().enumerate() {
            for (x,chr) in row.chars().enumerate() {
                if chr == '#' {rows[y][x] = true;}
            }
        }
        Ok(Self {id, rows, sides: [
            Side::from_chars(input[1].chars()),
            Side::from_chars(input[1..].iter().map( |x| x.chars().nth(TILE_SIZE-1).unwrap() )),
            Side::from_chars(input[TILE_SIZE].chars().rev()),
            Side::from_chars(input[1..].iter().map( |x| x.chars().next().unwrap() ).rev()),
        ]})
    }

    /// returnes a new array of transformed sides
    fn sides(&self, rotate: usize, flipped: bool) -> [Side; 4] {
        if flipped {[
            self.sides[(4 + rotate) % 4].get(flipped),
            self.sides[(7 + rotate) % 4].get(flipped),
            self.sides[(6 + rotate) % 4].get(flipped),
            self.sides[(5 + rotate) % 4].get(flipped),
        ]} else {[
            self.sides[(4 - rotate) % 4].get(flipped),
            self.sides[(5 - rotate) % 4].get(flipped),
            self.sides[(6 - rotate) % 4].get(flipped),
            self.sides[(7 - rotate) % 4].get(flipped),
        ]}
    }
    /// returns a new array of transformed rows.
    #[allow(clippy::needless_range_loop)]
    fn rows(&self, rotate: usize, flipped: bool) -> [[bool;TILE_SIZE];TILE_SIZE] {
        let mut res;
        if rotate % 4 == 0 {
            res = self.rows;
            if flipped { for x in res.iter_mut() { x.reverse(); }}
        } else if rotate % 4 == 2 {
            res = self.rows;
            res.reverse();
            if !flipped { for x in res.iter_mut() { x.reverse(); }}
        } else if rotate % 4 == 1 {
            res = [[false; TILE_SIZE];TILE_SIZE];
            if flipped {
                for x in 0..TILE_SIZE { for y in 0..TILE_SIZE { if self.rows[TILE_SIZE-1-y][TILE_SIZE-1-x] { res[x][y] = true; }}}
            } else {
                for x in 0..TILE_SIZE { for y in 0..TILE_SIZE { if self.rows[TILE_SIZE-1-y][x] { res[x][y] = true; }}}
            }
        } else /* rotate % 4 == 3*/ {
            res = [[false; TILE_SIZE];TILE_SIZE];
            if flipped {
                for x in 0..TILE_SIZE { for y in 0..TILE_SIZE { if self.rows[y][x] { res[x][y] = true; }}}
            } else {
                for x in 0..TILE_SIZE { for y in 0..TILE_SIZE { if self.rows[y][TILE_SIZE-1-x] { res[x][y] = true; }}}
            }               
        }
        res
    }
    fn transformed(&self, rotate: usize, flipped: bool) -> Self {
        Self {
            id: self.id,
            rows: self.rows(rotate, flipped),
            sides:self.sides(rotate, flipped),
        }
    }
}

struct JigsawPuzzle {
    tiles: HashMap<TileID, Tile>,
    tile_shapes: [Vec<TileID>; 3],
    size: usize,
    solution_part1: Vec<Vec<Tile>>,
    solution_part2: Vec<[usize;2]>,
    printed_px: Vec<[usize;2]>,
}
impl JigsawPuzzle {
    fn from_tiles(tile_vec: Vec<Tile>) -> Result<Self> {
        let size = match f32::sqrt(tile_vec.len() as f32) {
            x if !x.is_normal() || x.fract() != 0.0 => bail!("Invalid puzzle size sqrt({}): {}",tile_vec.len(), x),
            x => x as usize
        };
        let mut edge_count = HashMap::new();
        let mut tiles = HashMap::with_capacity(tile_vec.len());
        let mut tile_shapes = [Vec::new(), Vec::new(), Vec::new()];
        for tile in &tile_vec {
            for side in tile.sides {
                *edge_count.entry(side.shape).or_insert_with(|| 0) += 1;
                *edge_count.entry(side.rev  ).or_insert_with(|| 0) += 1;
            }
        }
        for tile in tile_vec {
            match &tile.sides.iter().filter(|side| edge_count[&side.shape] > 1).count() {
                2 => tile_shapes[0].push(tile.id),
                3 => tile_shapes[1].push(tile.id),
                4 => tile_shapes[2].push(tile.id),
                x => bail!("Tile {} has {} neighbours",tile.id, x),
            };
            tiles.insert(tile.id, tile);

        }  
        Ok(Self { tiles, tile_shapes, size, solution_part1: vec![vec![Tile::default();size]; size],solution_part2: Vec::new() ,printed_px: Vec::new() })
    }

    #[allow(clippy::collapsible_else_if)]
    fn solve_position(&mut self, x: usize, y:usize, available_tiles: [Vec<TileID>;3]) -> bool {
        if y >= self.size { return true; }
        let corner_edge_middle =  if x==0 || x==self.size-1 {
            if y == 0 || y==self.size-1 { 0 } else { 1 }
        } else {
            if y == 0 || y==self.size-1 { 1 } else { 2 }
        };
        for tile_id in &available_tiles[corner_edge_middle] {
            for flip in [false, true] { for rotate in 0..4 {
                let transformed_sides = self.tiles[tile_id].sides(rotate, flip);
                if (y == 0 || transformed_sides[0].rev == self.solution_part1[y-1][x].sides[2].shape)
                && (x == 0 || transformed_sides[3].rev == self.solution_part1[y][x-1].sides[1].shape) {
                    self.solution_part1[y][x] = self.tiles[tile_id].transformed(rotate, flip);
                    self.print_tile(x,y);
                    
                    let mut new_tiles = available_tiles.clone();
                    new_tiles[corner_edge_middle].retain(|tile| tile != tile_id);

                    if x == self.size-1 {
                        if self.solve_position(0,   y+1, new_tiles) { return true; }
                    } else {
                        if self.solve_position(x+1, y,   new_tiles) { return true; }
                    }
                }
            }}
        }
        self.solution_part1[y][x] = Tile::default();
        self.print_tile(x,y);
        false
    }

    fn solve(&mut self) -> bool { self.solve_position(0,0, self.tile_shapes.clone()) }
 
    fn find_monsters(&mut self) {
        // 8 variant of the monster when flipped / rotated
        let mut pix = Vec::new();
        for (monster,maxx, maxy) in [
            ([[19, 1],[18, 1],[18, 0],[17, 1],[16, 2],[13, 2],[12, 1],[11, 1],[10, 2],[ 7, 2],[ 6, 1],[ 5, 1],[ 4, 2],[ 1, 2],[ 0, 1]], 19, 2),
            ([[ 0, 1],[ 1, 1],[ 1, 0],[ 2, 1],[ 3, 2],[ 6, 2],[ 7, 1],[ 8, 1],[ 9, 2],[12, 2],[13, 1],[14, 1],[15, 2],[18, 2],[19, 1]], 19, 2),
            ([[19, 1],[18, 1],[18, 2],[17, 1],[16, 0],[13, 0],[12, 1],[11, 1],[10, 0],[ 7, 0],[ 6, 1],[ 5, 1],[ 4, 0],[ 1, 0],[ 0, 1]], 19, 2),
            ([[ 0, 1],[ 1, 1],[ 1, 2],[ 2, 1],[ 3, 0],[ 6, 0],[ 7, 1],[ 8, 1],[ 9, 0],[12, 0],[13, 1],[14, 1],[15, 0],[18, 0],[19, 1]], 19, 2),
            ([[ 1,19],[ 1,18],[ 0,18],[ 1,17],[ 2,16],[ 2,13],[ 1,12],[ 1,11],[ 2,10],[ 2, 7],[ 1, 6],[ 1, 5],[ 2, 4],[ 2, 1],[ 1, 0]],  2,19),
            ([[ 1, 0],[ 1, 1],[ 0, 1],[ 1, 2],[ 2, 3],[ 2, 6],[ 1, 7],[ 1, 8],[ 2, 9],[ 2,12],[ 1,13],[ 1,14],[ 2,15],[ 2,18],[ 1,19]],  2,19),
            ([[ 1,19],[ 1,18],[ 2,18],[ 1,17],[ 0,16],[ 0,13],[ 1,12],[ 1,11],[ 0,10],[ 0, 7],[ 1, 6],[ 1, 5],[ 0, 4],[ 0, 1],[ 1, 0]],  2,19),
            ([[ 1, 0],[ 1, 1],[ 2, 1],[ 1, 2],[ 0, 3],[ 0, 6],[ 1, 7],[ 1, 8],[ 0, 9],[ 0,12],[ 1,13],[ 1,14],[ 0,15],[ 0,18],[ 1,19]],  2,19),
        ] {
            for y in 0..self.size*TILE_SIZE2-maxy {
                'search: for x in 0..self.size*TILE_SIZE2-maxx {
                    for dx in 0..maxx { for dy in 0..maxy { self.print_px(x+dx,y+dy);}}
                    for [mx,my] in monster {
                        if !self.solution_part1[(y+my)/TILE_SIZE2][(x+mx)/TILE_SIZE2].rows[(y+my) % TILE_SIZE2 + 1][(x+mx) % TILE_SIZE2 + 1] {
                            self.clear_px();
                            pix.clear();
                            continue 'search;
                        }
                        pix.push([x+mx,y+my])
                    }
                    self.solution_part2.append(&mut pix);
                }
            }
        }
    }

    fn answer_part1(&self) -> u64 {
        self.solution_part1[0          ][0          ].id as u64 *
        self.solution_part1[0          ][self.size-1].id as u64 *
        self.solution_part1[self.size-1][0          ].id as u64 *
        self.solution_part1[self.size-1][self.size-1].id as u64
    }
    fn answer_part2(&self) -> usize {
        self.solution_part1.iter().map(|row|
             row.iter().map(|tile|
                tile.rows.get(1..TILE_SIZE-1).unwrap().iter().map(|row|
                    row.get(1..TILE_SIZE-1).unwrap().iter().filter(|x| **x).count() 
                ).sum::<usize>()
             ).sum::<usize>()
        ).sum::<usize>() 
        -
        self.solution_part2.len()
    }

    fn print_init(&self) {
        if !VERBOSE {return}
        for _ in 0..self.size*TILE_HALF {
            println!();
        }
        for x in 0..self.size {
            for y in 0..self.size {
                self.do_print_tile(x,y);
            }
        }
        std::io::stdout().flush().unwrap();
    }
    fn print_clear(&self) {
        if !VERBOSE {return}
        for _ in 0..self.size*TILE_HALF {
            up(1);
            print!("{:>1$}",' ',self.size*TILE_SIZE);
            left(self.size*TILE_SIZE);
        }
        std::io::stdout().flush().unwrap();
    }
    fn print_tile(&self, x: usize, y: usize) {
        if !VERBOSE {return}
        self.do_print_tile(x,y);
        std::io::stdout().flush().unwrap();
        sleep(Duration::from_millis(VERBOSE_SLEEP));
    }
    fn do_print_tile(&self, x: usize, y: usize) {
        let odd_even =  (x+y) % 2 == 0;
        up((self.size - y) *  TILE_HALF);
        right(x * TILE_SIZE);
        if self.solution_part1[y][x].id >= 0 {
            for y1 in 0..TILE_HALF {
                for x1 in 0..TILE_SIZE {
                    print_char(self.solution_part1[y][x].rows[y1*2][x1],false, self.solution_part1[y][x].rows[y1*2+1][x1], false, odd_even);
                }
                left(TILE_SIZE);
                down(1);
            }
        }
        else {
            color(odd_even);
            for _ in 0..TILE_HALF {
                print!("         ");
                left(TILE_SIZE);
                down(1);
            }
        }
        left(x*TILE_SIZE);
        down((self.size-1-y) * TILE_HALF);
        print!("\x1b[0m");
    }
    fn clear_px(&mut self) {
        if !VERBOSE { return }
        sleep(Duration::from_millis(VERBOSE_SLEEP2));
        
            for [x,y] in &self.printed_px {
                self.do_print_px(*x,*y,false);
            }
            std::io::stdout().flush().unwrap();
        
        self.printed_px.clear();

    }
    fn print_px(&mut self, x: usize, y: usize) {
        if !VERBOSE { return }
        self.printed_px.push([x,y]);
        self.do_print_px(x, y, true);
        std::io::stdout().flush().unwrap();
    }
    fn do_print_px(&self, x: usize, y: usize, hilight: bool) {
        let tile_x = x / TILE_SIZE2;
        let tile_y = y / TILE_SIZE2;
        let px_x = x % TILE_SIZE2+1;
        let px_y = y % TILE_SIZE2+1;



        up((self.size-tile_y) * TILE_HALF - px_y/2);
        std::io::stdout().flush().unwrap();

        right(tile_x * TILE_SIZE + px_x);
        std::io::stdout().flush().unwrap();

        if px_y % 2 == 0 {
            print_char(
                self.solution_part1[tile_y][tile_x].rows[px_y  ][px_x], hilight || self.solution_part2.contains(&[x,y]),
                self.solution_part1[tile_y][tile_x].rows[px_y+1][px_x], self.solution_part2.contains(&[x,y+1]),
                (tile_x+tile_y) % 2 == 0,
            );
        } else {
            print_char(
                self.solution_part1[tile_y][tile_x].rows[px_y-1][px_x], y > 0 &&self.solution_part2.contains(&[x,y-1]),
                self.solution_part1[tile_y][tile_x].rows[px_y  ][px_x], hilight || self.solution_part2.contains(&[x,y]),
                (tile_x+tile_y) % 2 == 0,
            );   
        }
        print!("\x1b[0m");

        std::io::stdout().flush().unwrap();

        down((self.size-tile_y) * TILE_HALF - px_y/2);
        left(tile_x * TILE_SIZE + px_x + 2);
        std::io::stdout().flush().unwrap();
    }
}
impl Puzzle {
    pub fn day20(&mut self) -> Result<()> {
        let mut jigsaw = JigsawPuzzle::from_tiles(self.input
            .split(|line| line.is_empty())
            .map(Tile::from_input)
            .collect::<Result<Vec<Tile>>>()?
        )?;
        jigsaw.print_init();
        if jigsaw.solve() {
            jigsaw.find_monsters();
            self.set_answer_a(jigsaw.answer_part1());
            self.set_answer_b(jigsaw.answer_part2());
        }
        jigsaw.print_clear();
        Ok(())
    }
}
