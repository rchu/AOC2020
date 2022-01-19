use std::env;
use std::io::{self, BufRead};
use std::fs::File;

mod days;
// #[derive(Debug)]
struct Input {
    day: u8,
    answer: Option<String>,
    input: Vec<String>,
}

fn read_input_file(file_name: &str) ->  Result<Input, Box<dyn std::error::Error>> {
    let mut lines = io::BufReader::new( File::open(file_name)?).lines();
    Ok(Input {
        day: if let Some(x) = lines.next() { x?.parse::<u8>()? } else { Err("Cannot read day; no lines in file")? },
        answer: if let Some(x) = lines.next() { x.ok().filter(|s| s!="")} else { Err("No first line to read")? },
        input: lines.into_iter().collect::<Result<Vec<_>, _>>()?,

    })
}
fn main() {

    println!("\x1b[33m* \x1b[32mAdvent of Code\x1b[33m *\x1b[0m\n");    
    for filename in env::args().skip(1) {
        let input = match read_input_file(&filename) {
            Ok(i) => i,
            Err(err) => {
                    println!("Problem with input file '{}': {}", filename, err);
                    continue;
            }
        };
        let output = match input.day {
            1 => days::day01(input.input),
            _ => "Not implemented".to_string(),
        };
        match input.answer {
            Some(x) if x == output => {println!("\x1b[32mDay {}\x1b[0m {} \x1b[37m({})\x1b[0m", input.day, output, filename);},
            None                   => {println!("\x1b[33mDay {}\x1b[0m {} \x1b[37m({})\x1b[0m", input.day, output, filename);}
            Some(answer)           => {println!("\x1b[31mDay {}\x1b[0m expect {} got {} \x1b[37m({})\x1b[0m", input.day, answer, output, filename);}, 
        }
    }
}
