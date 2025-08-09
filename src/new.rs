// SPDX-License-Identifier: MPL-2.0

/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use crate::file::*;

use std::env::set_current_dir;
use std::fs::*;
use std::io::ErrorKind;

fn new_project() -> Result<(), std::io::Error> {
    if !exists("project.ssg")? {
        create_dir("./site")?;
        create_dir("./out")?;

        create_file_from_str(
            "./project.ssg",
            "name: Example Website\n\
url: https://example.com/",
        );
    } else {
        set_current_dir("..")?;
        return Err(ErrorKind::AlreadyExists.into());
    }

    return Ok(());
}

pub fn new(name: String) -> Result<(), std::io::Error> {
    if !exists(&name)? {
        create_dir(&name)?;
    }

    set_current_dir(&name)?;
    new_project()?;

    return Ok(());
}
