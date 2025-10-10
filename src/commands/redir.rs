use crate::file::create_file_from_str;
use crate::interpreter_facilities::*;
use crate::misc::path_as_relative;

use std::fs::{create_dir_all, exists};
use std::path::PathBuf;
use std::{collections::HashMap, io::ErrorKind};

pub fn redir(
    origin: &[u8],
    last: &mut usize,
    current: &mut usize,
    vars: &mut HashMap<String, Vec<u8>>,
) -> Result<(), std::io::Error> {
    let mut to: PathBuf = path_as_relative(get_word_or_literal(origin, last, current)?);
    let mut to_dir: PathBuf = to.clone();

    if to.is_dir() {
        to.push("index.html");
    } else {
        to_dir.pop();
    }

    if !exists(to_dir.to_str().unwrap())? {
        create_dir_all(to_dir.to_str().unwrap())?;
    }

    let varpath: String = String::from_utf8(match vars.get("path") {
        Some(var) => var.to_vec(),
        _ => return Err(ErrorKind::Unsupported.into()),
    })
    .unwrap();

    create_file_from_str(
        to.to_str().unwrap(),
        &format!(
            "<!DOCTYPE html>
        <html>\
            <body>\
                <p>If the page does not automatically redirect...</p>\
                <p><a href=\"/{varpath}\">Click here</a></p>\
                <script>
                    window.location.href = \"/{varpath}\";
                </script>
            </body>\
        </html>
    "
        ),
    );

    Ok(())
}
