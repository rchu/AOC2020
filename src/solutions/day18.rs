use std::str::Chars;
use anyhow::{Result, anyhow, bail};
use crate::puzzle::Puzzle;

fn parse_math_part1(mut chars: Chars) -> Result<i64> {
    let mut num_rhs = None;
    let mut num_lhs = None;
    let mut opperation = None;
    let mut next_char = None;

    while let Some(char) = next_char.or_else(|| chars.next()) {
        next_char = None;
        // expect operator
        if num_rhs.is_some() && opperation.is_none() {
            if char == '+' { opperation = Some('+'); }
            else if char == '*' { opperation = Some('*'); }
            else { bail!("expect operation, got '{}'",char); }
            num_lhs = num_rhs;
            num_rhs = None;
        // else expect expression, a number...
        } else if char.is_numeric() {
            let mut str = String::from(char);
            loop {
                next_char = chars.next();
                match next_char {
                    None      => { num_rhs = Some(str.parse()?) },
                    Some('*') => { num_rhs = Some(str.parse()?) },
                    Some('+') => { num_rhs = Some(str.parse()?) },
                    Some(chr) => { str.push(chr); continue;},
                }
                break;
            }
        // ...or a () sub-expression
        } else if char == '(' {
            let mut str = String::new();
            let mut depth = 1;
            loop {
            
                match chars.next() {
                    Some('(') => {str.push('('); depth += 1; },
                    Some(')') => {str.push(')'); depth -= 1; },
                    Some(chr) => str.push(chr),
                    None      => return Err(anyhow!("{} unmatched closing bracket(s)",depth)),
                }
                if depth == 0 {
                    str.pop();
                    num_rhs = Some(parse_math_part1(str.chars())?);
                    break;
                }
            }
        } else {
            bail!("expect expression, got '{}'",char);
        }

        if num_lhs.is_some() && opperation.is_some() && num_rhs.is_some() {
            // perform operation
            num_rhs = Some(match opperation {
                Some('*') => num_lhs.unwrap() * num_rhs.unwrap(),
                Some('+') => num_lhs.unwrap() + num_rhs.unwrap(),
                _ => bail!("unknown operation, should never happen"),
            });
            opperation = None;
            num_lhs = None;
        }
    }
    num_rhs.ok_or_else(|| anyhow!("unexpected EOL"))
}
fn parse_math_part2(mut chars: Chars) -> Result<i64> {
    let mut muls = vec![];
    let mut num_rhs = None;
    let mut num_lhs = None;
    let mut opperation = None;
    let mut next_char = None;

    while let Some(char) = next_char.or_else(|| chars.next()) {
        next_char = None;
        // expect operator
        if num_rhs.is_some() && opperation.is_none() {
            if char == '+' { opperation = Some('+'); }
            else if char == '*' { opperation = Some('*'); }
            else { bail!("expect operation, got '{}'",char); }
            num_lhs = num_rhs;
            num_rhs = None;
        // else expect expression, a number...
        } else if char.is_numeric() {
            let mut str = String::from(char);
            loop {
                next_char = chars.next();
                match next_char {
                    None      => { num_rhs = Some(str.parse()?) },
                    Some('*') => { num_rhs = Some(str.parse()?) },
                    Some('+') => { num_rhs = Some(str.parse()?) },
                    Some(chr) => { str.push(chr); continue;},
                }
                break;
            }
        // ...or a () sub-expression
        } else if char == '(' {
            let mut str = String::new();
            let mut depth = 1;
            loop {
            
                match chars.next() {
                    Some('(') => {str.push('('); depth += 1; },
                    Some(')') => {str.push(')'); depth -= 1; },
                    Some(chr) => str.push(chr),
                    None      => return Err(anyhow!("{} unmatched closing bracket(s)",depth)),
                }
                if depth == 0 {
                    str.pop();
                    num_rhs = Some(parse_math_part2(str.chars())?);
                    break;
                }
            }
        } else {
            bail!("expect expression, got '{}'",char);
        }

        if num_lhs.is_some() && opperation.is_some() && num_rhs.is_some() {
            // perform operation
            match opperation {
                Some('+') => {
                    num_rhs = Some(num_lhs.unwrap() + num_rhs.unwrap())
                },
                Some('*') => {
                    muls.push(num_lhs.unwrap());
                },
                _ => bail!("unknown operation, should never happen"),
            };
            opperation = None;
            num_lhs = None;
        }
    }
    if !muls.is_empty() {
        Ok(muls.iter().product::<i64>() * num_rhs.unwrap_or(0))
    } else {
        num_rhs.ok_or_else(|| anyhow!("unexpected EOL"))
    }    
}

impl Puzzle {
    pub fn day18(&mut self) -> Result<()> {
        self.set_answer_a(
            self.input.iter()
                .map(|line| parse_math_part1(line.replace(' ',"").chars()))
                .collect::<Result<Vec<i64>>>()?
                .iter().sum::<i64>()
        );
        self.set_answer_b(
            self.input.iter()
                .map(|line| parse_math_part2(line.replace(' ',"").chars()))
                .collect::<Result<Vec<i64>>>()?
                .iter().sum::<i64>()
        );
        Ok(())
    }
}
