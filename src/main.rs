use std::env;

use std::io::BufRead;
use std::io;
use std::fs::File;

mod solutions;
pub struct Puzzle  {
    pub file: String,
    pub day: i32,
    pub answer_a: Option<String>,
    pub answer_b: Option<String>,
    pub input: Vec<String>,
    pub output_a: Option<String>,
    pub output_b: Option<String>,
}
impl Puzzle {
    pub fn from_file(file_name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut lines = io::BufReader::new( File::open(file_name)? ).lines();
        let items = match lines.next() {
            Some(Ok(x)) => x,
            Some(Err(x)) => return Err(Box::from(x)),
            None => return Err("no 1st line".into()),
        };
        let mut items = items//(|x| x.try_into()?);
            .split(',')
            .map(String::from)
            ;
        // let day=items.next().unwrap();
        Ok(Self {
            file: file_name.to_string(),
            day: items.next().unwrap().parse::<i32>()?,
            answer_a: items.next(),
            answer_b: items.next(),  
            input: lines.into_iter().collect::<Result<Vec<String>, _>>()?,
            output_a: None,
            output_b: None,
        })  
    }

    pub fn print_result(&self) {
        print!("\x1b[33m");
        let out_a = if let Some(out) = &self.output_a {
            if let Some(ans) = &self.answer_a {
                if ans == out {
                    print!("★");
                    format!("\x1b[32m'{}'\x1b[0m is correct",ans)
                } else {
                    print!(" ");
                    format!("'{}', got \x1b[31m{}\x1b[0m", ans, out)
                }
            } else {
                print!("☆");
                format!("\x1b[33m{}\x1b[0m may be correct", out)
            }
        } else {
            print!(" ");
            "got \x1b[31mnothing\x1b[0m".to_string()
        };
        let out_b = if let Some(out) = &self.output_b {
            if let Some(ans) = &self.answer_b {
                if ans == out {
                    print!("★");
                    format!("\x1b[32m'{}'\x1b[0m is correct",ans)
                } else {
                    print!(" ");
                    format!("'{}', got \x1b[31m{}\x1b[0m", ans, out)
                }
            } else {
                print!("☆");
                format!("\x1b[33m{}\x1b[0m may be correct", out)
            }
        } else {
            print!(" ");
            "got \x1b[31mnothing\x1b[0m".to_string()
        };

        println!("\x1b[32m Day {}\x1b[0m Part 1: {}, Part 2: {} (from {})\x1b[0m",self.day, out_a, out_b, self.file);
    }    
}

fn main() {
    if env::args().into_iter().count() < 2 {
        println!("Please tell me which file(s) to look at");
        return;
    } 
    println!("\x1b[33m* \x1b[32mAdvent of Code\x1b[33m *\x1b[0m\n");    
    for filename in env::args().skip(1) {
        match Puzzle::from_file(&filename) {
            Ok(mut puzzle) => {
                puzzle.solve();
                puzzle.print_result();
            },
            Err(err) => {
                println!("Problem with input file '{}': {}", filename, err);
            },
        };
    }
}
