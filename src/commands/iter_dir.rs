// SPDX-License-Identifier: MPL-2.0

/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use crate::docdata::*;
use crate::interpreter_facilities::*;
use crate::produce::*;

use std::collections::HashMap;
use std::fs::*;
use std::io::ErrorKind;

fn path_prod(
    cache: &mut HashMap<String, DocData>,
    vars: &HashMap<String, Vec<u8>>,
    anon_stack: &mut Vec<Vec<u8>>,
    path: &str,
    inner: &[u8],
    result: &mut Vec<u8>
) -> Result<(), std::io::Error> {
    let pathdata: Metadata = metadata(path)?;
    
    if pathdata.is_dir() {
        for file in read_dir(path)? {
            let name: String = file?.file_name().into_string().unwrap();
            let newpath: String = format!("{path}/{name}");

            let data: Metadata = metadata(&newpath)?;
            if data.is_dir() {
                path_prod(cache, vars, anon_stack, &newpath, inner, result)?;
            } else if data.is_file() {
                let mut new_vars: HashMap<String, Vec<u8>> = vars.clone();
                new_vars.insert("filepath".to_string(), newpath.as_bytes().to_vec());

                result.append(&mut produce(inner, &mut new_vars, cache, anon_stack)?);
            }
        }

        return Ok(());
    }

    return Err(ErrorKind::NotADirectory.into());
}

pub fn iter_dir(
    origin: &[u8],
    last: &mut usize,
    current: &mut usize,
    cache: &mut HashMap<String, DocData>,
    vars: &mut HashMap<String, Vec<u8>>,
    anon_stack: &mut Vec<Vec<u8>>,
) -> Result<Vec<u8>, std::io::Error> {
    let path: String = String::from_utf8(get_worl_produce(origin, current, last, vars, cache, anon_stack)?).unwrap();
    let inner: Vec<u8> = get_inner(origin, last, current)?;

    let mut result: Vec<u8> = Vec::new();

    path_prod(cache, vars, anon_stack, &path, &inner, &mut result)?;

    Ok(result)
}
