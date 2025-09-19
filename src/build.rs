// SPDX-License-Identifier: MPL-2.0

/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use crate::interpreter_facilities::*;
use crate::produce::*;
use crate::var_imports::*;

use std::collections::HashMap;
use std::fs::*;
use std::io::{ErrorKind, Read, Write};

fn template(
    path: &str,
    root_path: &str,
    name: &str,
) -> Result<(), std::io::Error> {
    // Open file for reading
    let file_path: String = format!("{path}/{name}");
    let mut file: File = File::open(&file_path)?;

    // Create destination directories
    let mut newpath = path.to_string();
    if newpath.starts_with("./") {
        newpath = newpath.replace("./", "");
    }

    let mut path_name: String = path.replace(root_path, "");
    path_name = format!("{path_name}/{name}");

    if path_name.starts_with("/") {
        path_name = path_name.replacen("/", "", 1);
    }

    newpath = "out/".to_string() + &newpath;
    if !exists(&newpath)? {
        create_dir_all(&newpath)?;
    }

    let dest_name: String = format!("{newpath}/{name}");

    // Read file into vector, produce result
    let mut data: Vec<u8> = Vec::new();
    file.read_to_end(&mut data)?;

    let result: Vec<u8>;
    if file_path.ends_with(".html") || file_path.ends_with(".htm") {
        data = to_normalized_vec(&data);

        let mut vars: HashMap<String, Vec<u8>> = get_vars_from_file("./project.ssg")?;
        vars.insert("path".to_string(), path_name.as_bytes().to_vec());

        let mut stack: Vec<Vec<u8>> = Vec::new();

        result = match produce(&data, &mut vars, &mut stack) {
            Err(err) => {
                eprintln!("Build failed! Error first provoked on file {file_path}");
                return Err(err);
            }

            Ok(data) => data,
        }
    } else {
        result = data;
    }

    // Write result
    let mut new: File = File::create(&dest_name)?;
    new.write(&result)?;

    return Ok(());
}

fn navigate_files(
    path: &str,
    root_path: &str,
) -> Result<(), std::io::Error> {
    let pathdata: Metadata = metadata(path)?;

    if pathdata.is_dir() {
        for dir in read_dir(path)? {
            let name: String = dir?.file_name().into_string().unwrap();
            let fpath: String = format!("{path}/{name}");

            let data: Metadata = metadata(&fpath)?;
            if data.is_dir() {
                navigate_files(&fpath, root_path)?;
            } else if data.is_file() {
                template(path, root_path, &name)?;
            }
        }

        return Ok(());
    }

    return Err(ErrorKind::NotADirectory.into());
}

pub fn build() -> Result<(), std::io::Error> {
    if exists("./project.ssg")? {
        remove_dir_all("./out")?;
        create_dir("./out")?;

        navigate_files("./site", "./site")?;
    } else {
        return Err(ErrorKind::AlreadyExists.into());
    }

    return Ok(());
}
