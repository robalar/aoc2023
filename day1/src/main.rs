fn main() {
    let input = include_str!("input.txt");

    let string_digits = [
        "one", 
        "two",
        "three", 
        "four", 
        "five", 
        "six",  
        "seven", 
        "eight",
        "nine",
        "ten",
    ];

    let answer: u32 = input.lines().map(|line| {
        let mut digits = line.chars().enumerate().filter_map(|(i, c)|{
            if c.is_numeric() {
                Some(c.to_digit(10).expect("could not conver char to digit"))
            } else {
                string_digits.iter().enumerate().find_map(|(val, digit_string)| line[i..].starts_with(digit_string).then_some((val + 1) as u32))
            }
        });
        
        let first_digit = digits.next().expect("no first digit");
        first_digit * 10 + digits.last().unwrap_or(first_digit)
    }).sum();

    dbg!(answer);
}

