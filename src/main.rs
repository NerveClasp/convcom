use regex::Regex;
use std::env;
use std::io::Error;
use std::process::exit;
use std::process::Command;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let commit_message = match args.len() {
        x if x > 1 => args[1].clone(),
        _ => String::from(""),
    };
    let git_branch_name = Command::new("git")
        .arg("branch")
        .arg("--show-current")
        .output()?;
    if !git_branch_name.status.success() {
        println!("Could not get current git branch name! Are you in a git repo right now?");
        exit(1);
    }
    let branch_name = String::from_utf8(git_branch_name.stdout).unwrap();
    // ^(\w+)\/((?:[A-Z]+)?-?\d+)-?(.+)
    // ^(\w+)\/([A-Z]*-?\d+)?-?(.+)
    // something like `feature/NC-0-some-branch-name`
    // let branch_name_regex = Regex::new(r"^(\w+)/((?:[A-Z]*)?-?\d+)-?(.+)").unwrap();
    let branch_name_regex = Regex::new(r"^(\w+)/([A-Z]*-?\d+)?-?(.+)").unwrap();
    let caps = branch_name_regex.captures(&branch_name).unwrap();

    let prefix = caps.get(1).map_or("", |m| m.as_str());
    let ticket_number = caps.get(2).map_or("", |m| m.as_str());
    let rest_of_branch = caps.get(3).map_or("", |m| m.as_str());
    let clean_rest_of_branch = str::replace(rest_of_branch, "-", " ");

    println!(
        "{} || {} || {} || {}",
        prefix, ticket_number, clean_rest_of_branch, commit_message
    );

    let commit_type = match prefix {
        "feature" => "feat",
        "feat" => "feat",
        "fix" => "fix",
        "bugfix" => "fix",
        "quickfix" => "fix",
        _ => "",
    };

    let mut comm_message: Vec<char> = match commit_message.len() {
        x if x == 0 => clean_rest_of_branch.chars().collect(),
        _ => commit_message.chars().collect(),
    };
    comm_message[0] = comm_message[0].to_lowercase().nth(0).unwrap();
    let final_commit_message: String = comm_message.into_iter().collect();
    let result_string = format!(
        "{}: {} ({})",
        commit_type, final_commit_message, ticket_number
    );

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
