// SPDX-License-Identifier: MPL-2.0

/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use crate::interpreter_facilities::*;

use std::collections::HashMap;

pub fn set(
    origin: &[u8],
    current: &mut usize,
    last: &mut usize,
    vars: &mut HashMap<String, Vec<u8>>,
    anon_stack: &mut Vec<Vec<u8>>,
) -> Result<(), std::io::Error> {
    if !is_eof(origin, *current) {
        let name: String = get_word(origin, last, current)?;
        let value: Vec<u8> = get_worl_produce(origin, current, last, vars, anon_stack)?;

        vars.insert(name, value);
    }

    return Ok(());
}
