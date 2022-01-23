use std::collections::HashSet;
use std::collections::HashMap;



pub fn day01(input: Vec<String>) -> (Option<String>, Option<String>) {
    let numbers: Vec<u32> = input
        .iter()
        .map(|x|{x.parse::<u32>().expect("cannot convert input into u32")})
        .collect();
    
    let mut answer1 = 0u32;
    'outer1: for i1 in 0..numbers.len() {
        for i2 in (i1+1)..numbers.len() {
            if numbers[i1] + numbers[i2] == 2020 {
                answer1 = numbers[i1] * numbers[i2];
                break 'outer1;
            }
        }
    }

    let mut answer2 = 0u32;
    'outer2: for i1 in 0..(numbers.len()) {
        for i2 in (i1+1)..(numbers.len()) {
            for i3 in (i2+1)..numbers.len() {
                if numbers[i1] + numbers[i2] + numbers[i3] == 2020 {
                    answer2 = numbers[i1] * numbers[i2] * numbers[i3];
                    break 'outer2;
                } 
            }
        }
    }

    (Some(answer1.to_string()), Some(answer2.to_string()))
}

pub fn day02(input: Vec<String>) -> (Option<String>, Option<String>) {
    let mut valid_count_1 = 0u16;
    let mut valid_count_2 = 0u16;
    for line in input {
        let mut num1:usize = 0;
        let mut num2:usize = 0;
        let mut pos: usize = 0;
        let mut chr= ' ';
        for (i,ch) in line.chars().enumerate() {
            if ch == '-' {
                num1 = line.get(0..i)
                    .unwrap()
                    .parse::<usize>()
                    .unwrap_or_else(|_| panic!("Invalid range_start 0..{} in line {}",i,line));
                pos = i+1;
            } else if ch == ' ' {
                num2 = line.get(pos..i)
                    .unwrap()
                    .parse::<usize>()
                    .unwrap_or_else(|_| panic!("Invalid range_end {}..{} in line {}",pos,i,line));
                chr = line.chars().nth(i+1).unwrap();
                pos = i+3;
                break;
            }
        }
        let chr_count = line.get(pos..).unwrap_or("").chars().filter(|x| x == &chr).count();
        valid_count_1 += ((num1 <= chr_count) & (chr_count <= num2)) as u16;
        valid_count_2 += (
            (line.chars().nth(pos+num1).unwrap() == chr) ^
            (line.chars().nth(pos+num2  ).unwrap() == chr)
        ) as u16
    }
    (Some(valid_count_1.to_string()), Some(valid_count_2.to_string()))
}

pub fn day03(input: Vec<String>) -> (Option<String>, Option<String>) {
    struct Map {
        map: Vec<Vec<char>>,
        width: usize,
        height: usize,
    }
    impl Map {
        fn from(str_map:&[String]) -> Self {
            let char_map: Vec<Vec<char>> = str_map.iter().map(|x| x.chars().collect()).collect();
            Self {
                width: char_map[0].len(),
                height: char_map.len(),
                map: char_map,
            }
        }
        fn get(&self, x: usize, y: usize) -> Option<char> {
            if y >= self.height {
                None
            } else {
                Some(self.map[y][x % self.width])
            }
        }
    }
    let map = Map::from(&input);
    let slopes: Vec<usize> = [(1,1), (3,1), (5,1), (7,1), (1,2)]
        .into_iter()
        .map(|(dx,dy)| (1..map.height)
            .filter(|&i| Some('#') == map.get(i*dx, i*dy) )
            .count()
        ).collect();
    (Some(slopes[1].to_string()), Some(slopes.into_iter().product::<usize>().to_string()))
}

pub fn day04(input: Vec<String>) -> (Option<String>, Option<String>) {
    fn between<T: std::cmp::PartialOrd>(value: T, low: T, high: T) -> bool { (low <= value) && (value <= high) }
    
    let mut valid_count_1 = 0;
    let mut valid_count_2 = 0;

    let mut passport = HashMap::new();
    let mut input_iter = input.iter();

    loop {
        let line = input_iter.next();
        if line == Some(&String::from("")) || line == None {
            if passport.len() == (7 + passport.contains_key("cid") as usize) {
                valid_count_1 += 1;
                valid_count_2 += passport.into_iter().all(|(key,val):(&str,&str)| -> bool {match key {
                    "cid" => true,
                    "byr" => between(val.parse::<i32>().unwrap_or(0), 1920, 2002),
                    "iyr" => between(val.parse::<i32>().unwrap_or(0), 2010, 2020),
                    "eyr" => between(val.parse::<i32>().unwrap_or(0), 2020, 2030),
                    "ecl" => ["amb","blu","brn","gry","grn","hzl","oth"].into_iter().any(|x| x==val),
                    "pid" => (val.chars().count() == 9) && (val.parse::<i32>().is_ok()),
                    "hgt" => {
                        if let Some((idx,_)) = val.char_indices().rev().nth(1) {
                            match val.get(idx..) {
                                Some("in") => between(val.get(0..idx).and_then(|x| x.parse::<i32>().ok()).unwrap_or(0),  59,  76),
                                Some("cm") => between(val.get(0..idx).and_then(|x| x.parse::<i32>().ok()).unwrap_or(0), 150, 193),
                                _ => false,
                            }
                        } else { false }
                    }
                    "hcl" => {
                        let mut chrs = val.chars();
                        (chrs.next() == Some('#')) && chrs.all(|x| "0123456789abcdef".chars().any(|y| x==y))
                    }
                    key => {
                        println!("Unknown attribute {}:{}",key,val);
                        false
                    },
                }}) as i32;
            }
            if line == None { break; }
            passport = HashMap::new();

        } else {  
            for item in line.unwrap().split(' ').map(|x| x.split(':').collect::<Vec<&str>>()) {
                passport.insert(item[0],item[1]);
            }
        }
    }
    (Some(valid_count_1.to_string()), Some(valid_count_2.to_string()))
}

pub fn day05(input: Vec<String>) -> (Option<String>, Option<String>) {
    let mut max_seat_id: i32 = 0;
    let seats: Vec<i32> = input.iter().map( |line| {
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
    (Some(max_seat_id.to_string()), Some(my_seat_id.to_string()))
}

pub fn day06(input: Vec<String>) -> (Option<String>, Option<String>) {
    const ALPHABET: [char; 26] = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];
    let mut input_iter = input.iter();
    let mut any_sum = 0;
    let mut all_sum = 0;
    let mut any_yes = HashSet::new();
    let mut all_yes = Vec::from(ALPHABET);
    loop {
        let line = input_iter.next();
        if line == Some(&String::from("")) || line == None {
            any_sum += any_yes.len();
            all_sum += all_yes.len();
            any_yes.clear();
            all_yes = Vec::from(ALPHABET);
            if line == None { break; }
        } else {
            line.unwrap().chars().for_each( |x| { any_yes.insert(x); } );
            all_yes = line.unwrap().chars().filter( |x| all_yes.contains(x) ).collect();
        }
    }
    (Some(any_sum.to_string()), Some(all_sum.to_string()))
}

pub fn day07(input: Vec<String>) -> (Option<String>, Option<String>) {
    fn bag_contains_gold(rules: &HashMap<String,Vec<(i32,String)>>, bag: &str) -> bool {
        for sub_bag in rules.get(bag).unwrap_or(&Vec::new()) {
            if sub_bag.1 == "shiny gold" || bag_contains_gold(rules, &sub_bag.1) { return true; }
        }
        false
    }
    fn count_bags(rules: &HashMap<String,Vec<(i32,String)>>, bag: &str) -> i32 {
        let mut count = 1;
        for sub_bag in rules.get(bag).unwrap_or(&Vec::new()) {
            count += sub_bag.0 * count_bags(rules, &sub_bag.1);
        }
        count
    }
    
    let mut rules: HashMap<String,Vec<(i32,String)>> = HashMap::new();
    for mut words in input.iter().map(|x| x.split(' ').peekable()) {
        let bag = format!("{} {}",words.next().unwrap(), words.next().unwrap());
        let mut contain = Vec::<(i32,String)>::new();
        words.next();
        words.next();
        while words.peek().is_some() {
            if words.peek() == Some(&"no") { break; }
            contain.push((
                words.next().map(|x| x.parse::<i32>().expect("invalid number")).unwrap(),
                format!("{} {}", words.next().unwrap(), words.next().unwrap()),
            ));
            words.next();
        }
        rules.insert(bag, contain);
    };
    
    (
        Some(rules.keys().filter(|x| bag_contains_gold(&rules, x) ).count().to_string()),
        Some((count_bags(&rules, &String::from("shiny gold")) -1).to_string()),
    )
}

pub fn day08(input: Vec<String>) -> (Option<String>, Option<String>) {
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
                _ => break,
            }
            *instruction = String::from("");
        }
        if line as usize == input.len() {
            RunResult::Terminate(val)
        } else {
            RunResult::Loop(val, line)
        }
    
    }
    let result1 = match run(input.clone()) {
        RunResult::Loop(val,_) => val,
        _ => -1,
    };

    let mut result2 = -1;
    for (idx, val) in input.iter().enumerate() {
        let mut fixed = input.clone();
        fixed[idx as usize] = match (val.get(..3),val.get(4..))  {
            (Some("jmp"), Some(i)) => format!("nop {}", i),
            (Some("nop"), Some(i)) => format!("jmp {}", i),
            _ => continue,
        };
        if let RunResult::Terminate(i) = run(fixed) {
            result2 = i;
            break;
        }
    }
    (Some(result1.to_string()), Some(result2.to_string()))
}