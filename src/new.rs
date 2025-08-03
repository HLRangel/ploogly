// SPDX-License-Identifier: MPL-2.0

/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use crate::file::*;

use std::env::set_current_dir;
use std::fs::*;
use std::io::{Write, stdin, stdout, ErrorKind};

fn new_project() -> Result<(), std::io::Error>{
    if !exists("project.ssg")? {
    create_dir("./site")?;
    create_dir("./out")?;

    create_file_from_str("./project.ssg", 
"name: Example Website\n\
url: https://example.com/");

    } else {
        set_current_dir("..")?;
        return Err(ErrorKind::AlreadyExists.into());
    }

    return Ok(());
}

fn message_line_input(msg: &str) -> Result<String, std::io::Error> {
    let mut conf: String = String::new();
    
    print!("{msg}");

    stdout().flush()?;
    stdin().read_line(&mut conf)?;

    return Ok(conf);
}


pub fn new(name: String) -> Result<(), std::io::Error> {
    if !exists(&name)? {
        create_dir(&name)?;
    }

    set_current_dir(&name)?;
    new_project()?;

    return Ok(());
}