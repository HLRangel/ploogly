// SPDX-License-Identifier: MPL-2.0

/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use crate::interpreter_facilities::*;
use std::collections::HashMap;

pub fn unset(
    origin: &[u8],
    last: &mut usize,
    current: &mut usize,
    vars: &mut HashMap<String, Vec<u8>>,
) -> Result<(), std::io::Error> {
    let to_unset: String = get_word_or_literal(origin, last, current)?;
    if vars.contains_key(&to_unset) {
        vars.remove(&to_unset);
    }

    return Ok(());
}
