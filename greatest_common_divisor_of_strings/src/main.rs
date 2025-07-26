fn main() {
    println!("Amogus");

    let test_cases = [
        ("ABCABC".to_string(), "ABC".to_string(), "ABC".to_string()),
        ("ABABAB".to_string(), "ABAB".to_string(), "AB".to_string()),
        ("LEET".to_string(), "CODE".to_string(), "".to_string())
    ];

    for (str1, str2, result) in test_cases {
        // assert_eq!(gcd_of_strings(str1, str2), result);
        gcd_of_strings(str1, str2);
    }
}

pub fn gcd_of_strings(str1: String, str2: String) -> String {
    // let chars = str1.chars();
    // let chars_iter = for x in chars {
    //     println!("{}", x);
    // };

    // let mut max_found_divisor = "";

    // Maybe it is not correct str1 can be 1 char
    // let str1_chars_count = str1.chars().count();
    // let str2_chars_count = str2.chars().count();
    //
    // if is_odd(str1_chars_count) || is_odd(str2_chars_count) {
    //     return "".to_string();
    // }

    let mut max_found_divisor = "".to_string();

    // let shortest_passed_string = if str1.len() > str2.len() { str2.clone() } else { str1.clone() };

    // println!("{} {} {}", str1, str2, shortest_passed_string);


    let Strings { longer_string, shorter_string } = determine_length_of_strings(str1, str2);

    println!("longer string: {longer_string}");
    println!("shorter string: {shorter_string}");

    let mut longer_passed_string_iter = longer_string
        .chars()
        .into_iter();

    let max_prefix_found: String = shorter_string
        .chars()
        .into_iter()
        .take_while(|char_from_shorter_string| {
            let longer_passed_string_char = longer_passed_string_iter.next();

            return match longer_passed_string_char {
                Some(char_from_longer_string) => {
                    println!("char from longer string: {}", char_from_longer_string);
                    println!("char from shorter string: {}", char_from_shorter_string);

                    if char_from_longer_string.eq(char_from_shorter_string) {
                        // It's redundant since we can collect string in the end
                        max_found_divisor.push(char_from_longer_string);
                        true
                    } else {
                        false
                    }
                },
                None => false
            };
        })
        .collect();

    // We are finding only the max prefix, but we are losing shorter prefixes
    // Maybe use HashMap, where the key will be String, and the value bool
    println!("max prefix found: {}", max_prefix_found);



    "".to_string()
}

fn is_odd(chars_count: usize) -> bool {
    chars_count % 2 == 1
}

pub struct Strings {
    longer_string: String,
    shorter_string: String
}

fn determine_length_of_strings(str1: String, str2: String) -> Strings {
    if str1.len() > str2.len() {
        Strings {
            longer_string: str1,
            shorter_string: str2
        }
    } else {
        Strings {
            longer_string: str2,
            shorter_string: str1
        }
    }
}