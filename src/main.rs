use regex::Regex;

fn main() {
    let text = String::from("feature/BC-123-yo-stream");
    let test_message_raw = "Test commit message";
    // (\w+)\/([A-Z]*-?\d+)
    let type_re = Regex::new(r"^(\w+)").unwrap(); // TODO: figure out something better
    let ticket_re = Regex::new(r"([A-Z]*-?\d+)").unwrap();
    let prefix = type_re.find(&text).unwrap().as_str();
    let ticket_number = ticket_re.find(&text).unwrap().as_str();
    let message_type = match prefix {
        "feature" => "feat",
        "feat" => "feat",
        "fix" => "fix",
        "bugfix" => "fix",
        "quickfix" => "fix",
        _ => "",
    };
    let mut v: Vec<char> = test_message_raw.chars().collect();
    v[0] = v[0].to_lowercase().nth(0).unwrap();
    let test_message: String = v.into_iter().collect();

    let result_string = format!("{}: {} ({})", message_type, test_message, ticket_number);

    println!("Result: {}", result_string);
}
