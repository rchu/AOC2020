mod puzzle;
mod day1_6;
mod day7_10;
mod day11_12;
mod day13_;
use puzzle::Puzzle;
use std::env::args;
use anyhow::Result;

fn main() -> Result<()> {
    println!("\x1b[33m* \x1b[32mAdvent of Code\x1b[33m *\x1b[0m\n");    
    if args().count() < 2 {
        println!("Please tell me which file(s) to look at");
    } 
    for arg in args().skip(1){
        Puzzle::from_file(&arg)?
        .solve()?
        .print_result()
    }
    Ok(())
}
