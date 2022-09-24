use regex_engine::RegExp;

fn main() {
    let regex = "a|(bc)*".to_string();
    let regexp = RegExp::new(regex);

    let input = "a".to_string();
    let result = regexp.matches(input);
    println!("result: {}", result);
}
