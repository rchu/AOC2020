

pub fn day01(input: Vec<String>) -> String {
    let numbers: Vec<u32> = input
    .into_iter()
    .map(|x|{x.parse::<u32>().expect("cannot covert input into u32")})
    .collect();
    
    let mut answer1 =  0u32;
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

   format!("{},{}",answer1, answer2)
}
