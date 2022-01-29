use crate::puzzle::Puzzle;

impl Puzzle {
    pub fn day03(&mut self) {
        struct Map {
            map: Vec<Vec<char>>,
            width: usize,
            height: usize,
        }
        impl Map {
            fn from(str_map:&[String]) -> Self {
                let char_map: Vec<Vec<char>> = str_map.iter().map(|x| x.chars().collect()).collect();
                Self {
                    width: char_map[0].len(),
                    height: char_map.len(),
                    map: char_map,
                }
            }
            fn get(&self, x: usize, y: usize) -> Option<char> {
                if y >= self.height {
                    None
                } else {
                    Some(self.map[y][x % self.width])
                }
            }
        }
        let map = Map::from(&self.input);
        let slopes: Vec<usize> = [(1,1), (3,1), (5,1), (7,1), (1,2)]
            .into_iter()
            .map(|(dx,dy)| (1..map.height)
                .filter(|&i| Some('#') == map.get(i*dx, i*dy) )
                .count()
            ).collect();
        self.set_answer_a(slopes[1]);
        self.set_answer_b(slopes.into_iter().product::<usize>());
    }
}