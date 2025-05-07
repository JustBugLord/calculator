use std::collections::HashMap;
use std::process::exit;
use std::str::FromStr;
use regex::Regex;

const NUMBERS: &str = "0123456789.";
const ANOMALY_REGEX_EXPRESSION: &str = "[^0-9*+-/()^]+";
const OPERATORS: [(char, (u8, fn(isize, isize) -> isize)); 5] = [
    ('+', (1, |first, second| first + second)),
    ('-', (1, |first, second| first - second)),
    ('*', (2, |first, second| first * second)),
    ('/', (2, |first, second| { if second == 0 { println!("Divide by zero"); exit(0) } else { first / second } })),
    ('^', (3, |first, second| first.pow(second as u32))),
];

fn higher_precedence(first_op: &char, second_op: &char) -> bool {
    let operator_precedence: HashMap<char, (u8, fn(isize, isize) -> isize)> = HashMap::from(OPERATORS);
    operator_precedence.get(first_op)
        .expect(format!("Not found operator in precedence map: {}", first_op).as_str())
        >=
        operator_precedence.get(second_op)
            .expect(format!("Not found operator in precedence map: {}", first_op).as_str())
}

fn infix_to_postfix(expression: &str) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    let mut stack: Vec<char> = Vec::new();
    let mut number: Vec<char> = Vec::new();

    for (idx, symbol) in expression.chars().enumerate() {
        if NUMBERS.contains(symbol) {
            number.push(symbol);
        } else {
            if !number.is_empty() {
                output.push(number.iter().collect());
                number.clear();
            }

            if symbol == '(' {
                stack.push(symbol);
            } else if symbol == ')' {
                while !stack.is_empty() && stack[stack.len() - 1] != '(' {
                    output.push(stack.pop().expect("Failed to pull an item from the stack").to_string())
                }
                stack.pop();
            } else {
                if symbol == '-' {
                    number.push(symbol);
                    if idx != 0 && NUMBERS.contains(expression.chars().nth(idx - 1).expect("Couldn't extract the char")) {
                        stack.push('+');
                    }
                } else {
                    while !stack.is_empty()
                        && !"()".contains(stack[stack.len() - 1])
                        && higher_precedence(&stack[stack.len() - 1], &symbol) {
                        output.push(stack.pop().expect("Failed to pull an item from the stack").to_string())
                    }
                    stack.push(symbol);
                }
            }
        }
    }
    if !number.is_empty() {
        output.push(number.iter().collect());
        number.clear();
    }
    while !stack.is_empty() {
        output.push(stack.pop().expect("Failed to pull an item from the stack").to_string())
    }
    output
}

fn evaluate_postfix(postfix_expression: &Vec<String>) -> isize {
    let mut stack: Vec<String> = Vec::new();
    for token in postfix_expression {
        if token.len() == 1 {
            match OPERATORS.iter().position(|&x| token.chars().next().expect("") == x.0) {
                Some(idx) => {
                    let (second, first) = (
                        isize::from_str(stack.pop().expect("Failed to pull an item from the stack").as_str()).expect("Couldn't convert string to isize"),
                        isize::from_str(stack.pop().expect("Failed to pull an item from the stack").as_str()).expect("Couldn't convert string to isize")
                    );
                    let result = OPERATORS[idx].1.1(first, second);
                    stack.push(result.to_string())
                }
                None => stack.push(token.clone())
            }
        } else {
            stack.push(token.clone())
        }
    }
    isize::from_str(stack.pop().expect("Failed to pull an item from the stack").as_str()).expect("Couldn't convert string to isize")
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() >= 2 {
        let anomaly_regex: Regex = Regex::new(ANOMALY_REGEX_EXPRESSION)
            .expect(format!("Anomaly regex expression parsing error: {}", ANOMALY_REGEX_EXPRESSION).as_str());
        println!("Expression: {:?}", args[1]);
        let expression = args[1].clone().replace(" ", "");
        if anomaly_regex.is_match(&expression) {
            println!("An anomaly was found in the expression: {}", &expression)
        }
        println!("Result: {:?}", evaluate_postfix(&infix_to_postfix(expression.as_str())));
    }
}
