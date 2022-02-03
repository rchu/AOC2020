
use std::{io::BufRead, time::Instant};
use std::io;
use std::fs::File;
use anyhow::{anyhow, Result, Error, Context, bail};

pub struct Puzzle  {
    pub file: String,
    pub day: i32,
    pub answer_a: Option<String>,
    pub answer_b: Option<String>,
    pub input: Vec<String>,
    pub output_a: Option<String>,
    pub output_b: Option<String>,
    pub time: f64,

}
impl Puzzle {
    pub fn from_file(file_name: &str) -> Result<Self> {
        let mut lines = io::BufReader::new( File::open(file_name)? ).lines();
        let mut items = lines.next().ok_or(anyhow!("Input file contains no lines"))??
            .split(',')
            .map(String::from)
            .collect::<Vec<String>>()
            .into_iter();
        Ok(Self {
            file: file_name.to_string(),
            day: items.next().unwrap().parse::<i32>().context("Unable to parse puzzle number from input file")?,
            answer_a: items.next(),
            answer_b: items.next(),  
            input: lines.collect::<Result<Vec<String>, _>>()?,
            output_a: None,
            output_b: None,
            time: f64::NAN,

        })  
    }

    pub fn print_result(&self) {
        let (star_a, out_a) = if let Some(out) = &self.output_a {
            if let Some(ans) = &self.answer_a {
                if ans == out {
                    ('★', format!("\x1b[32m'{}'\x1b[0m is correct",ans))
                } else {
                    (' ', format!("'{}', got \x1b[31m{}\x1b[0m", ans, out))
                }
            } else {
                ('☆', format!("\x1b[33m{}\x1b[0m may be correct", out))
            }
        } else {
            (' ', "got \x1b[31mnothing\x1b[0m".to_string())
        };
        let (star_b, out_b) = if let Some(out) = &self.output_b {
            if let Some(ans) = &self.answer_b {
                if ans == out {
                    ('★', format!("\x1b[32m'{}'\x1b[0m is correct",ans))
                } else {
                    (' ', format!("'{}', got \x1b[31m{}\x1b[0m", ans, out))
                }
            } else {
                ('☆', format!("\x1b[33m{}\x1b[0m may be correct", out))
            }
        } else {
            (' ', "got \x1b[31mnothing\x1b[0m".to_string())
        };
        
        println!(
            "{:>6.3}s \x1b[33m{}{} \x1b[32mDay {:02}\x1b[0m Part 1: {}, Part 2: {} (from {})\x1b[0m",
            self.time, star_a, star_b, self.day, out_a, out_b, self.file
        );
    }  

    pub fn get_input_as<T: std::str::FromStr>(&self) -> Result<Vec<T>> {
        self.input.iter().map( |x| x
            .parse::<T>()
            .map_err(|_| Error::msg(format!("invalid <{}>: '{}' while parsing input", std::any::type_name::<T>(), x )) )
        ).collect()
    }
    pub fn set_answer_a<T:std::string::ToString>(&mut self, answer: T) { self.output_a = Some(answer.to_string()); }
    pub fn set_answer_b<T:std::string::ToString>(&mut self, answer: T) { self.output_b = Some(answer.to_string()); }

    pub fn solve(&mut self) -> Result<&Self>  {
        let now = Instant::now();
        match self.day {
             1 => self.day01(),
             2 => self.day02(),
             3 => self.day03(),
             4 => self.day04(),
             5 => self.day05(),
             6 => self.day06(),
             7 => self.day07(),
             8 => self.day08(),
             9 => self.day09()?,
            10 => self.day10()?,
            11 => self.day11()?,
            12 => self.day12a().and_then(|_| self.day12b())?,
            13 => self.day13a().and_then(|_| self.day13b())?,
            14 => self.day14()?,
            15 => self.day15()?,
            16 => self.day16()?,
            17 => self.day17()?,
            _ => bail!("No solver found for day {}",self.day),
        }
        self.time = now.elapsed().as_secs_f64();
        Ok(self)
    }
}