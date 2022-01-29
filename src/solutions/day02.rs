use crate::puzzle::Puzzle;

impl Puzzle {
    pub fn day02(&mut self) {
        let mut valid_count_1 = 0;
        let mut valid_count_2 = 0;
        for line in self.input.iter() {
            let mut num1:usize = 0;
            let mut num2:usize = 0;
            let mut pos: usize = 0;
            let mut chr= ' ';
            for (i,ch) in line.chars().enumerate() {
                if ch == '-' {
                    num1 = line.get(0..i)
                        .unwrap()
                        .parse::<usize>()
                        .unwrap_or_else(|_| panic!("Invalid range_start 0..{} in line {}",i,line));
                    pos = i+1;
                } else if ch == ' ' {
                    num2 = line.get(pos..i)
                        .unwrap()
                        .parse::<usize>()
                        .unwrap_or_else(|_| panic!("Invalid range_end {}..{} in line {}",pos,i,line));
                    chr = line.chars().nth(i+1).unwrap();
                    pos = i+3;
                    break;
                }
            }
            let chr_count = line.get(pos..).unwrap_or("").chars().filter(|x| x == &chr).count();
            valid_count_1 += ((num1 <= chr_count) & (chr_count <= num2)) as i32;
            valid_count_2 += (
                (line.chars().nth(pos+num1).unwrap() == chr) ^
                (line.chars().nth(pos+num2  ).unwrap() == chr)
            ) as i32
        }
        self.set_answer_a(valid_count_1);
        self.set_answer_b(valid_count_2);
    }
}