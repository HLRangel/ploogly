// SPDX-License-Identifier: MPL-2.0

/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

// Handles the small syntax for retrieving values from context files

use crate::interpreter_facilities::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

pub fn read_text_arg(origin: &[u8], current: &mut usize) -> Vec<u8> {
    let mut vecmap: Vec<u8> = Vec::new();

    while !is_eof(origin, *current) && origin[*current] != b'\n' {
        match origin[*current] {
            b'|' => {
                if !is_eof(origin, *current + 1) && origin[*current + 1] == b'\n' {
                    *current += 2;
                } else {
                    vecmap.push(origin[*current]);
                    *current += 1;
                }
            },

            b'>' => {
                if is_three_rarrow(origin, *current) && 
                    is_char_n(origin, *current + 3, b'\n') {
                    *current += 4;
                    vecmap.push(b'\n');
                } else {
                    vecmap.push(origin[*current]);
                    *current += 1;
                }
            },

            _ => {
                vecmap.push(origin[*current]);
                *current += 1;
            }
        }
    }

    return vecmap;
}

// Import variables with primitive yaml-like format, <name>: <var> <nl / |>
pub fn import_variables(origin: &[u8]) -> Result<HashMap<String, Vec<u8>>, std::io::Error> {
    let mut vars: HashMap<String, Vec<u8>> = HashMap::new();
    let mut current: usize = 0;

    while current < origin.len() {
        match origin[current] {
            b':' => {
                let varname: String = highlight_word_ltr(origin, current);
                current += 1;

                to_next_notwp(origin, &mut current);

                let vecmap: Vec<u8> = read_text_arg(origin, &mut current);
                vars.insert(varname, vecmap);
            }

            _ => {
                current += 1;
            }
        }
    }

    return Ok(vars);
}

pub fn get_vars_from_file(path: &str) -> Result<HashMap<String, Vec<u8>>, std::io::Error> {
    let mut fhandle = File::open(path)?;
    let mut vecy: Vec<u8> = Vec::new();
    
    fhandle.read_to_end(&mut vecy)?;
    vecy = to_normalized_vec(&vecy);

    return Ok(import_variables(&vecy)?);
}