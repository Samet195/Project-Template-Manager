//! Project Template Manager (Commands)

use std::env::{current_dir, temp_dir};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

use crate::{data, tools::extract};

/// Use command
pub fn use_cmd(template: String, path: Option<String>, git: bool) {
    let index = data::INDEX.to_vec();
    let data = data::DATA.to_vec();
    let tmp_path = format!("{}/ptm.tmp", temp_dir().display());

    // Provides default value for path
    let path = if path.is_none() {
        PathBuf::from(current_dir().unwrap().display().to_string())
    } else {
        let path = PathBuf::from(path.unwrap());
        if path.exists() {
            path
        } else {
            fs::create_dir(&path).unwrap();
            path
        }
    };

    // Checks is template exist
    if index.contains(&template.as_str()) {
        let id = index.binary_search(&template.as_str()).unwrap();

        // Create temp file
        fs::write(&tmp_path, data[id]).unwrap();

        // Extract files
        if path.as_path().is_dir() {
            match extract(fs::File::open(&tmp_path).unwrap(), path.clone()) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error: {:?}", e);
                }
            }

            // Initialize git repository
            if git {
                match Command::new("git")
                    .args(["init", path.to_str().unwrap()])
                    .output()
                {
                    Ok(_) => {}
                    Err(_) => eprintln!("Git installition is not found."),
                }
            }
        } else {
            eprintln!("\"{}\" is not valid path.", path.display())
        }

        // Remove temp file
        fs::remove_file(&tmp_path).unwrap();
    } else {
        eprintln!(
            "Error: \"{}\" is not avaible template.\nAvaible templates:",
            template
        );
        for i in index {
            eprintln!("    {}", i);
        }
    }
}

/// List command
pub fn list_cmd() {
    let index = data::INDEX;

    println!("Avaible templates:");
    for i in index {
        println!("    {}", i);
    }
}
