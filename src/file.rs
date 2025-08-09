// SPDX-License-Identifier: MPL-2.0

/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use crate::docdata::*;
use crate::interpreter_facilities::to_normalized_vec;
use crate::md2html::*;
use crate::produce::*;

use std::collections::HashMap;
use std::fs::*;
use std::io::ErrorKind;
use std::io::{Read, Write};

pub fn file_produce_and_append(
    toap: &mut File,
    result: &mut Vec<u8>,
    vars: &HashMap<String, Vec<u8>>,
    cache: &mut HashMap<String, DocData>,
    anon_stack: &mut Vec<Vec<u8>>,
) -> Result<(), std::io::Error> {
    let mut fdata: Vec<u8> = Vec::new();
    toap.read_to_end(&mut fdata)?;

    fdata = to_normalized_vec(&fdata);

    let mut hmap: HashMap<String, Vec<u8>> = vars.clone();

    result.append(&mut produce(&fdata, &mut hmap, cache, anon_stack)?);

    return Ok(());
}

/*
pub fn append_from_file(toap: &mut File,
                    result: &mut Vec<u8>
                ) -> Result<(), std::io::Error> {
    let mut fdata: Vec<u8> = Vec::new();
    toap.read_to_end(&mut fdata)?;

    result.append(&mut fdata);

    return Ok(());
} */

pub fn create_file_from_str(path: &str, text: &str) {
    let mut file: File = File::create(path).expect("Failed to create file!");

    file.write_all(text.as_bytes())
        .expect("Failed to write to file!");
}

pub fn inclusion_into_result(
    result: &mut Vec<u8>,
    vars: &HashMap<String, Vec<u8>>,
    cache: &mut HashMap<String, DocData>,
    anon_stack: &mut Vec<Vec<u8>>,
    arg: &str,
) -> Result<(), std::io::Error> {
    if arg.ends_with(".html") || arg.ends_with(".htm") {
        if exists(format!("./templates/{arg}"))? {
            let mut fhandle: File = File::open(format!("./templates/{arg}"))?;

            file_produce_and_append(&mut fhandle, result, vars, cache, anon_stack)?;

            return Ok(());
        } else if exists(&arg)? {
            let mut fhandle: File = File::open(&arg)?;

            file_produce_and_append(&mut fhandle, result, vars, cache, anon_stack)?;

            return Ok(());
        }
    } else if arg.ends_with(".md") || arg.ends_with(".markdown") {
        if exists(&arg)? {
            let mut fhandle: File = File::open(&arg)?;

            let mut fdata: Vec<u8> = Vec::new();
            fhandle.read_to_end(&mut fdata)?;

            fdata = to_normalized_vec(&fdata);

            fdata = to_md(&fdata)?;

            result.append(&mut fdata);

            return Ok(());
        }
    }

    return Err(ErrorKind::NotSeekable.into());
}
