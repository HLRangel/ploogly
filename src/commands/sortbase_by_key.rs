// SPDX-License-Identifier: MPL-2.0

/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use crate::interpreter_facilities::*;
use crate::docdata::*;
use crate::bases::data::*;

use std::collections::HashMap;
use std::io::ErrorKind;

pub fn sortbase_by_key(
    origin: &[u8],
    result: &mut Vec<u8>,
    current: &mut usize,
    last: &mut usize,
    vars: &mut HashMap<String, Vec<u8>>,
    cache: &mut HashMap<String, DocData>,
    anon_stack: &mut Vec<Vec<u8>>,
) -> Result<(), std::io::Error> {
    if !is_eof(origin, *current) {
	let ext: String = get_word_or_literal(origin, last, current)?;
        let mut base: Base = base_from_json(&get_worl_produce(origin, current, last, vars, cache, anon_stack)?)?;

	base_sort_by_key(&mut base, &ext)?;

	result.append(&mut base.to_json()?);
    }

    return Ok(());
}
