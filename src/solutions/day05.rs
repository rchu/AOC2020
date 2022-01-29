use crate::puzzle::Puzzle;

impl Puzzle {
    pub fn day05(&mut self) {
        let mut max_seat_id: i32 = 0;
        let seats: Vec<i32> = self.input.iter().map( |line| {
            let mut number: i32 = 0;
            for chr in line.chars().into_iter().map(|x| (x == 'B')||(x=='R')) {
                number = (number << 1) | chr as i32;
            }
            if number > max_seat_id { max_seat_id = number; }
            number 
        }).collect();
        
        let mut nr = 1;
        let my_seat_id = loop {
            if seats.contains(&(nr-1)) && !seats.contains(&(nr)) && seats.contains(&(nr+1)) { break nr; }
            if nr >= 1024 { break -1; } else { nr += 1; }
        };
        self.set_answer_a(max_seat_id);
        self.set_answer_b(my_seat_id);
    }
}