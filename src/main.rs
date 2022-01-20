use std::env;
use std::io::{self, BufRead};
use std::fs::File;

mod solutions;
pub struct Puzzle {
    pub file: String,
    pub day: u8,
    pub answer: Option<String>,
    pub input: Vec<String>,
    pub output: Option<String>,
}
impl Puzzle {
    pub fn from_file(file_name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut lines = io::BufReader::new( File::open(file_name)?).lines();
        Ok(Self {
            file: file_name.to_string(),
            day: if let Some(x) = lines.next() { x?.parse::<u8>()? } else { Err("Cannot read day; no lines in file")? },
            answer: if let Some(x) = lines.next() { x.ok().filter(|s| s!="")} else { Err("No first line to read")? },
            input: lines.into_iter().collect::<Result<Vec<String>, _>>()?,
            output: None,
        })  
    }

    pub fn print_result(&self) {
        if self.output.is_none() {
            println!("\x1b[31mDay {}\x1b[0m No output \x1b[37m(from {})\x1b[0m", self.day, self.file);
        } else if self.answer.is_none() {
            println!("\x1b[33mDay {}\x1b[0m '\x1b[33m{}\x1b[0m' may be right \x1b[37m(from {})\x1b[0m", self.day, self.output.as_ref().unwrap(), self.file);
        } else if self.answer == self.output {
            println!("\x1b[32mDay {}\x1b[0m '\x1b[33m{}\x1b[0m' is correct \x1b[37m(from {})\x1b[0m", self.day, self.output.as_ref().unwrap(), self.file);
        } else {
            println!("\x1b[31mDay {}\x1b[0m '\x1b[33m{}\x1b[0m' is wrong, should be '\x1b[33m{}\x1b[0m' \x1b[37m(from {})\x1b[0m", self.day, self.output.as_ref().unwrap(), self.answer.as_ref().unwrap(), self.file);
        }
    }    
}

fn main() {
    if env::args().into_iter().count() < 2 {
        println!("Please tell me which file(s) to look at");
        ()
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
