use crate::puzzle::Puzzle;
impl Puzzle {
    pub fn day08(&mut self) {
        enum RunResult {
            Terminate(i32),
            Loop(i32,i32),
        }
        fn run(mut input: Vec<String>) -> RunResult {
            let mut line = 0i32;
            let mut val = 0i32;
            while let Some(instruction) = input.get_mut(line as usize) {
                match (instruction.get(..3),instruction.get(4..).and_then(|x| x.parse::<i32>().ok()))  {
                    (Some("acc"), Some(i)) => { line += 1; val += i; },
                    (Some("jmp"), Some(i)) => { line += i; },
                    (Some("nop"), Some(_)) => { line += 1; },
                    _ => { return RunResult::Loop(val, line); },
                }
                *instruction = String::from("");
            }
            RunResult::Terminate(val)
        }
        
        self.set_answer_a(match run(self.input.clone()) {
            RunResult::Loop(val,_) => val,
            _ => -1,
        });

        for (idx, val) in self.input.iter().enumerate() {
            let mut fixed = self.input.clone();
            fixed[idx as usize] = match (val.get(..3),val.get(4..))  {
                (Some("jmp"), Some(i)) => format!("nop {}", i),
                (Some("nop"), Some(i)) => format!("jmp {}", i),
                _ => continue,
            };
            if let RunResult::Terminate(i) = run(fixed) {
                self.set_answer_b(i);
                break;
            }
        }
    }
}