// SPDX-License-Identifier: MPL-2.0

/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use crate::docdata::*;
use crate::interpreter_facilities::*;
use crate::produce::*;
use crate::bases::data::*;

use std::collections::HashMap;

pub fn iterant(
    result: &mut Vec<u8>,
    base: &Base,
    toiter: &[u8],
    cache: &mut HashMap<String, DocData>,
    vars: &mut HashMap<String, Vec<u8>>,
    anon_stack: &mut Vec<Vec<u8>>
) -> Result<(), std::io::Error> {
    for entry in &base.bases {
	let mut new_vars: HashMap<String, Vec<u8>> = vars.clone();
	let ctx: HashMap<String, Vec<u8>> = entry_into_hashmap(&entry)?;

	new_vars.extend(ctx);

	new_vars.insert("docdata".to_string(), entry_produced(entry)?);

	result.append(&mut produce(toiter, &mut new_vars, cache, anon_stack)?);
    }

    Ok(())
}

pub fn iter_base(
    origin: &[u8],
    result: &mut Vec<u8>,
    last: &mut usize,
    current: &mut usize,
    cache: &mut HashMap<String, DocData>,
    vars: &mut HashMap<String, Vec<u8>>,
    anon_stack: &mut Vec<Vec<u8>>
) -> Result<(), std::io::Error> {
    let mut base: Base = base_from_json(&get_worl_produce(origin, current, last, vars, cache, anon_stack)?)?;
    
    let inner: Vec<u8> = get_inner(origin, last, current)?;

    iterant(result, &mut base, &inner, cache, vars, anon_stack)?;

    Ok(())
}
