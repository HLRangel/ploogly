// SPDX-License-Identifier: MPL-2.0

/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use crate::file::inclusion_into_result;
use crate::interpreter_facilities::*;
use crate::var_imports::*;

use std::collections::HashMap;

fn interpret_var_or_stack_push(
    vec: &Vec<Vec<u8>>,
    vars: &mut HashMap<String, Vec<u8>>,
    anon_stack: &mut Vec<Vec<u8>>,
) -> Result<(), std::io::Error> {
    for arg in vec.iter().rev() {
        if arg.contains(&b':') {
            vars.extend(import_variables(&arg)?);
        } else {
            anon_stack.push(arg.to_vec());
        }
    }

    return Ok(());
}

pub fn template(
    origin: &[u8],
    last: &mut usize,
    current: &mut usize,
    vars: &mut HashMap<String, Vec<u8>>,
    anon_stack: &mut Vec<Vec<u8>>,
) -> Result<Vec<u8>, std::io::Error> {
    let filename: String = get_word_or_literal(origin, last, current)?;
    let vec: Vec<Vec<u8>> = get_separated_arguments(origin, last, current, vars, anon_stack)?;

    let mut thisstack: Vec<Vec<u8>> = anon_stack.clone();
    let mut thisvars: HashMap<String, Vec<u8>> = vars.clone();
    let mut result: Vec<u8> = Vec::new();

    interpret_var_or_stack_push(&vec, &mut thisvars, &mut thisstack)?;

    inclusion_into_result(&mut result, &thisvars, &mut thisstack, &filename)?;

    return Ok(result);
}
