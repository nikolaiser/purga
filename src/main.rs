use clap::Parser;
use serde::Serialize;
use serde_json::json;
use std::collections::HashMap;
use std::error::Error;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Flake input that will be used to provide the arguments
    #[arg(short, long, default_value = "purgaArgs")]
    input: String,
    /// Key-value pairs to pass to the flake. Format is '--arg name1=value1 --arg name2=value2'
    /// If you provide the same key multiple times it will be passed to the flake as an array
    #[arg(short, long = "arg", number_of_values = 1, value_parser = parse_key_val::<String, String>)]
    args: Vec<(String, String)>,
    /// Nix command to call
    #[clap(name = "--", trailing_var_arg = true, allow_hyphen_values = true)]
    nix_command: Vec<String>,
}

/// Parse a single key-value pair
fn parse_key_val<T, U>(s: &str) -> Result<(T, U), Box<dyn Error + Send + Sync + 'static>>
where
    T: std::str::FromStr,
    T::Err: Error + Send + Sync + 'static,
    U: std::str::FromStr,
    U::Err: Error + Send + Sync + 'static,
{
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{s}`"))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}

#[derive(Serialize, Debug)]
#[serde(untagged)]
enum PurgaValue {
    Str(String),
    StrVec(Vec<String>),
}

fn transform(input: Vec<(String, String)>) -> HashMap<String, PurgaValue> {
    let mut map = HashMap::new();

    for (key, value) in input {
        match map.get_mut(&key) {
            Some(PurgaValue::Str(existing)) => {
                // If the key already exists and it's a single String, convert to a vector
                let mut vec = Vec::new();
                vec.push(existing.clone());
                vec.push(value);
                map.insert(key, PurgaValue::StrVec(vec));
            }
            Some(PurgaValue::StrVec(vec)) => {
                // If the key exists and it's already a vector of strings, just push the new value
                vec.push(value);
            }
            None => {
                // If the key does not exist, insert it as a single String
                map.insert(key, PurgaValue::Str(value));
            }
        }
    }

    map
}

fn main() {
    let cli_args = Args::parse();
    let args: HashMap<String, PurgaValue> = transform(cli_args.args);

    let json_args = json!(args);

    let command = format!(
        "f=$(mktemp);echo '{}' > $f;{} --override-input {} file+file://$f;rm -rf $f",
        json_args,
        cli_args.nix_command.join(" "),
        cli_args.input
    );

    let mut process = Command::new("/bin/sh")
        .arg("-c")
        .arg(command)
        .spawn()
        .expect("Nix command failed");

    let status = process.wait().expect("Failed to wait for command");

    if !status.success() {
        eprintln!("Command failed with exit code: {}", status);
        std::process::exit(1);
    }
}
