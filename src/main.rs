fn main() {
    count_up_to(100);
}

fn fizzbuzz(number: u32)-> String {
    match(number % 3, number % 5) {
        (0, 0) => "fizzbuzz".to_string(),
        (0, _) => "fizz".to_string(),
        (_, 0) => "buzz".to_string(),
        (_, _) => number.to_string()
    }
}


fn count_up_to (number: u32) -> () {
    for i in 1..=number {
        println!("{}", fizzbuzz(i))
    }
}


#[cfg(test)]
mod tests {
    use super::fizzbuzz;
    #[test]
    fn test_fizzbuzz() {
        assert_eq!(fizzbuzz(3), "fizz");
        assert_eq!(fizzbuzz(5), "buzz");
        assert_eq!(fizzbuzz(15), "fizzbuzz");
        assert_eq!(fizzbuzz(4), "4");
    }
}