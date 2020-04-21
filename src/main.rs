use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::exit;

use clap::{App, Arg, SubCommand};
use regex::Regex;

mod main_tests;

const REPLACE_TAG_COMMAND: &str = "rt";
const SERVICE_NAME_ARGUMENT: &str = "service-name";
const TAG_ARGUMENT: &str = "tag";

const ERROR_EXIT_CODE: i32 = 1;

fn main() {
    let mut command_matched = false;

    let matches = App::new("Docker Compose File Tool")
        .version("0.1.2")
        .about("Multi-tool for docker-compose.yml file")
        .subcommand(SubCommand::with_name(REPLACE_TAG_COMMAND)
            .about("replace image tag for service")
            .arg(
            Arg::with_name(SERVICE_NAME_ARGUMENT)
                   .value_name(SERVICE_NAME_ARGUMENT)
                   .takes_value(true).required(true)
            )
            .arg(
                Arg::with_name(TAG_ARGUMENT)
                    .value_name(TAG_ARGUMENT)
                    .takes_value(true).required(true)
            )
        ).get_matches();

    match matches.subcommand_matches(REPLACE_TAG_COMMAND) {
        Some(args) => {
            command_matched = true;

            let service_name = args.value_of(SERVICE_NAME_ARGUMENT).unwrap();
            let tag = args.value_of(TAG_ARGUMENT).unwrap();

            let compose_file_path = Path::new("docker-compose.yml");

            if replace_tag_for_service_image(compose_file_path, service_name, tag) {
                println!("service image tag successfully replaces")

            } else {
                println!(
                    "unable to replace service image tag, possible reason - invalid service name"
                );
                exit(ERROR_EXIT_CODE);
            }
        }
        None => {}
    }

    if !command_matched {
        println!("use dcft --help");
        exit(ERROR_EXIT_CODE)
    }
}

fn replace_tag_for_service_image(compose_file_path: &Path, service_name: &str, tag: &str) -> bool {
    println!("replace image tag to '{}' for service '{}'", &tag, &service_name);
    let mut result = false;

    let compose_file = File::open(compose_file_path)
                                  .expect("unable to open docker-compose.yml file");

    let buffered = BufReader::new(&compose_file);

    let mut inside_service_section = false;

    let service_row_pattern = get_service_row_regex(service_name);
    let tag_row_pattern = get_tag_row_regex();

    let mut result_content = String::new();

    for line in buffered.lines() {
        let row = line.unwrap();

        let mut flag = false;

        if !result && inside_service_section && tag_row_pattern.is_match(&row) {
            println!("image row has been found");
            let groups = tag_row_pattern.captures_iter(&row).next().unwrap();

            let current_tag = String::from(&groups[1]);

            println!("current tag: '{}'", &current_tag);

            let replace_result = row.replace(&current_tag, tag);

            let row_to_write = format!("{}\n", &replace_result);
            result_content.push_str(&row_to_write);

            result = true;

            flag = true;
        }

        if !result && !inside_service_section && service_row_pattern.is_match(&row) {
            println!("service '{}' has been found", service_name);
            inside_service_section = true;
        }

        if !flag {
            let row_to_write = format!("{}\n", &row);
            result_content.push_str(&row_to_write);
        }
    }

    fs::write(compose_file_path, &result_content)
        .expect("unable to write to docker-compose.yml file");

    return result;
}

fn get_service_row_regex(service_name: &str) -> Regex {
    let pattern = format!("^[ \t]+({}):", service_name);
    return Regex::new(&pattern).unwrap()
}

fn get_tag_row_regex() -> Regex {
    return Regex::new("\\s+image: .*/.*:(.*)").unwrap()
}
