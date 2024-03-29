mod puzzle;
mod solutions;
use puzzle::Puzzle;
use std::env::args;


fn main() {
    println!("\x1b[33m* \x1b[32mAdvent of Code\x1b[33m *\x1b[0m\n");    
    if args().count() < 2 {
        println!("Please tell me which file(s) to look at");
    } 
    let mut total_time = 0.;
    for arg in args().skip(1){
        match Puzzle::from_file(&arg) {
            Ok(mut p) => match p.solve() {
                Ok(p) => { total_time += p.time; p.print_result() },
                Err(e) => println!("  \x1b[32m Day {}\x1b[0m \x1b[31mError: {}\x1b[0m", p.day, e),
            },
            Err(e) =>  println!("  \x1b[32m Day ??\x1b[0m {}: \x1b[31mError: {}\x1b[0m",arg, e),
        };
    }
    println!("total time: {:>6.3}s", total_time);
}
