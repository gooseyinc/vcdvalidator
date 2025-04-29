use clap::Parser;
use std::fs::File;
use std::io::{BufReader, Read};
use regex::Regex;
use serde_json::json;

/// VCD Validator
#[derive(Parser)]
struct Args {
    #[clap(short, long)]
    file: String,

    #[clap(short, long)]
    json: bool,
}

fn main() {
    let args = Args::parse();

    let file = File::open(&args.file).expect("Could not open the file.");
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).expect("Could not read the file.");

    if contents.trim().is_empty() {
        let output = json!({
            "valid": false,
            "reason": "The file is empty",
        });

        if args.json {
            println!("{}", serde_json::to_string_pretty(&output).unwrap());
        } else {
            println!("{}", output);
        }
        std::process::exit(1);
    }

    // Regex check
    let date_re = Regex::new(r"\$date\s+.*?\s+\$end").unwrap();
    let timescale_re = Regex::new(r"\$timescale\s+.*?\s+\$end").unwrap();
    let scope_re = Regex::new(r"\$scope\s+.*?\s+\$end").unwrap();
    let var_re = Regex::new(r"\$var\s+.*?\s+\$end").unwrap();
    let enddefs_re = Regex::new(r"\$enddefinitions\s+\$end").unwrap();
    let time_val_re = Regex::new(r"#\d+\s*\n(b[01xXzZ]+|[01xXzZ])\s+\S+").unwrap();

    // Semantic check
    let mut order = Vec::new();
    for cap in Regex::new(r"\$(\w+)").unwrap().captures_iter(&contents) {
        order.push(cap[1].to_lowercase());
    }

    let required_order = vec!["date", "timescale", "scope", "var", "enddefinitions"];
    let mut valid_order = true;

    for (i, val) in required_order.iter().enumerate() {
        if let Some(pos) = order.iter().position(|x| x == val) {
            if pos < i {
                valid_order = false;
                break;
            }
        } else {
            valid_order = false;
            break;
        }
    }

    let json_output = json!({
        "valid": date_re.is_match(&contents)
            && timescale_re.is_match(&contents)
            && scope_re.is_match(&contents)
            && var_re.is_match(&contents)
            && enddefs_re.is_match(&contents)
            && time_val_re.is_match(&contents)
            && valid_order,
        "blocks": {
            "date": date_re.is_match(&contents),
            "timescale": timescale_re.is_match(&contents),
            "scope": scope_re.is_match(&contents),
            "var": var_re.is_match(&contents),
            "enddefinitions": enddefs_re.is_match(&contents),
            "time_value": time_val_re.is_match(&contents)
        },
        "order_valid": valid_order
    });

    if args.json {
        println!("{}", serde_json::to_string_pretty(&json_output).unwrap());
    } else {
        println!("{}", json_output);
    }
}
