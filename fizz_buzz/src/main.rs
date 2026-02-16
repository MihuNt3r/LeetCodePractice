struct Solution;

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let numbers = 1..=n;

        numbers
            .into_iter()
            .map(|number| get_string_for_number(number))
            .collect()
    }
}

fn get_string_for_number(number: i32) -> String {
    match number {
        number if number % 3 == 0 && number % 5 == 0 => "FizzBuzz".to_string(),
        number if number % 3 == 0 => "Fizz".to_string(),
        number if number % 5 == 0 => "Buzz".to_string(),
        number=> number.to_string()
    }
}

fn main() {

}
