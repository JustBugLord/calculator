use regex::Regex;

const ANOMALY_REGEX_EXPRESSION: &str = "[^0-9*+-/()]+";

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() >= 2 {
        let anomaly_regex: Regex = Regex::new(ANOMALY_REGEX_EXPRESSION)
            .expect(format!("Anomaly regex expression parsing error: {}", ANOMALY_REGEX_EXPRESSION).as_str());
        let expression = args[1].clone().replace(" ", "");
        if anomaly_regex.is_match(&expression) {
            println!("An anomaly was found in the expression: {}", &expression)
        }
        // calculate
    }
}
