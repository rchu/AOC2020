mod puzzle;
mod day1_6;
mod day7_;
use std::env;

fn main() {
    if env::args().into_iter().count() < 2 {
        println!("Please tell me which file(s) to look at");
        return;
    } 
    println!("\x1b[33m* \x1b[32mAdvent of Code\x1b[33m *\x1b[0m\n");    
    for filename in env::args().skip(1) {
        match puzzle::Puzzle::from_file(&filename) {
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
