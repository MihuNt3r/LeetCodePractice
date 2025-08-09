fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut parentheses_stack: Vec<char> = Vec::new();

        let opening_bracket_closing_bracket: Vec<(char, char)> = vec![
            ('(', ')'),
            ('{', '}'),
            ('[', ']'),
        ];

        for parentheses in s.chars() {
            match parentheses {
                '(' | '{' | '[' => {
                    parentheses_stack.push(parentheses);
                },
                closing_bracket => {
                    let opening_bracket = opening_bracket_closing_bracket
                        .iter()
                        .find(|(_, closing)| closing_bracket == *closing)
                        .map(|(opening_bracket, _)| opening_bracket)
                        .unwrap();

                    if parentheses_stack.last() != Some(&opening_bracket) {
                        parentheses_stack.push(closing_bracket);
                    } else if parentheses_stack.last() == Some(&opening_bracket) {
                        parentheses_stack.pop();
                    }
                }
            }
        }

        parentheses_stack.is_empty()
    }
}
