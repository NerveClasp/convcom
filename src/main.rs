use regex::Regex;
use std::env;
use std::io::Error;
use std::process::exit;
use std::process::Command;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let commit_message = args[1].clone();
    println!("{:?}", commit_message);
    let output = Command::new("git")
        .arg("branch")
        .arg("--show-current")
        .output()?;
    if !output.status.success() {
        println!("Could not get current git branch name! Are you in a git repo right now?");
        exit(1);
    }
    let branch_name = String::from_utf8(output.stdout).unwrap();
    println!("{:?}", branch_name.trim());
    // (\w+)\/([A-Z]*-?\d+)
    let type_re = Regex::new(r"^(\w+)").unwrap(); // TODO: figure out something better
    let ticket_re = Regex::new(r"([A-Z]*-?\d+)").unwrap();
    let prefix = type_re.find(&branch_name).unwrap().as_str();
    let ticket_number = ticket_re.find(&branch_name).unwrap().as_str();
    let message_type = match prefix {
        "feature" => "feat",
        "feat" => "feat",
        "fix" => "fix",
        "bugfix" => "fix",
        "quickfix" => "fix",
        _ => "",
    };
    let mut v: Vec<char> = commit_message.chars().collect();
    v[0] = v[0].to_lowercase().nth(0).unwrap();
    let test_message: String = v.into_iter().collect();

    let result_string = format!("{}: {} ({})", message_type, test_message, ticket_number);

    println!("Result: {}", result_string);
    let final_result = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(result_string)
        .output()?;

    if !final_result.status.success() {
        let error_message = String::from_utf8(final_result.stdout).unwrap();

        println!("Error: {}", error_message);
        exit(1);
    }

    Ok(())
}
