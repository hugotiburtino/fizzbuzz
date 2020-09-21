use std::io::Write;

fn fizzbuzz(number: u32) -> String {
    match (number % 3, number % 5) {
        (0, 0) => "fizzbuzz".to_string(),
        (0, _) => "fizz".to_string(),
        (_, 0) => "buzz".to_string(),
        (_, _) => number.to_string(),
    }
}

pub fn count_up_to(number: u32, writer: &mut dyn Write) -> () {
    for i in 1..=number {
        let _ = writeln!(writer, "{}", fizzbuzz(i));
    }
}

pub fn parse_input(input: String) -> u32 {
    let number: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("{} is not a positive number", input.trim()),
    };
    number
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fizzbuzz() {
        assert_eq!(fizzbuzz(3), "fizz");
        assert_eq!(fizzbuzz(5), "buzz");
        assert_eq!(fizzbuzz(15), "fizzbuzz");
        assert_eq!(fizzbuzz(4), "4");
    }
    #[test]
    #[should_panic(expected = "is not a positive number")]
    fn test_parse_input() {
        parse_input("-50".to_string());
        parse_input("NaN causes error".to_string());
    }
}
