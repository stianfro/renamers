use clap::Parser;
use regex::Regex;
use std::fs;
use std::fs::read_dir;
use std::path::Path;
use std::process;

/// Program to rename directories with optional prefix
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Optional prefix to rename directories
    #[clap(short, long)]
    prefix: Option<String>,
}

fn main() {
    let args = Args::parse();

    // Get list of directories in the current path
    let entries = read_dir(".").unwrap_or_else(|err| {
        eprintln!("Error reading directories: {}", err);
        process::exit(1);
    });

    let mut filtered_dirs = vec![];
    for entry in entries {
        match entry {
            Ok(entry) => {
                if entry.file_type().map(|f| f.is_dir()).unwrap_or(false) {
                    filtered_dirs.push(entry.path());
                }
            }
            Err(err) => {
                eprintln!("Error accessing entry: {}", err);
            }
        }
    }

    if filtered_dirs.is_empty() {
        println!("No directories found in the current path. Exiting.");
        process::exit(1);
    }

    // Regular expression to match prefix and numeric suffix
    let re = Regex::new(r"^(.+?)[_ ](\d+)$").unwrap();

    for dir in filtered_dirs {
        let dir_name = dir.file_name().unwrap().to_string_lossy();
        println!("Processing directory: {}", dir_name);

        if let Some(caps) = re.captures(&dir_name) {
            let extracted_prefix = &caps[1];
            let number = &caps[2];

            // Determine final prefix
            let final_prefix = match &args.prefix {
                Some(p) => p.clone(),
                None => extracted_prefix.to_string(),
            };

            let new_name = format!("{}_{}", final_prefix.replace(" ", "_"), number);

            // Check if the target name already exists
            if Path::new(&new_name).exists() {
                println!("Warning: Target directory {} already exists. Skipping {}.", new_name, dir_name);
            } else {
                println!("Renaming {} to {}", dir_name, new_name);
                if let Err(err) = fs::rename(&dir, &new_name) {
                    eprintln!("Error renaming {}: {}", dir_name, err);
                }
            }
        } else {
            println!("No match for directory: {}, skipping.", dir_name);
        }
    }
}

