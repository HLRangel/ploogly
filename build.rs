use std::io::ErrorKind;
use std::process::{Command, Output};

fn git_exists() -> bool {
    let out: Output = match Command::new("git")
	.arg("--version")
	.output() {
	    Ok(out) => {
		out
	    },
	    Err(_) => {
		return false;
	    }
	};

    if !out.status.success() {
	return false;
    }

    true
}

fn get_commit_string() -> Result<String, std::io::Error> {
    if git_exists() {
	let out: Output = Command::new("git")
	    .arg("log")
	    .arg("-n1")
	    .arg("--format=\"%H\"")
	    .arg("-n")
	    .arg("1").output()?;
	
	return Ok(String::from_utf8(out.stdout).unwrap())
    }

    Err(ErrorKind::NotFound.into())
}


fn main() {
    let ver: String = match get_commit_string() {
	Ok(st) => st,
	Err(_) => {
	    println!("cargo::warn=git did not run successfully\
		      using dummy commit ID");

	    "abcdefghij".to_string()
	}
    };

    println!("cargo::warning=Building Ploogly {}", &ver[1..7]);
    println!("cargo::rustc-env=PLOOGLY_COMMIT_ID={}", &ver[1..7]);
}
