// SPDX-License-Identifier: MPL-2.0

/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/
use std::collections::HashMap;

use crate::interpreter_facilities::*;
use crate::bases::data::*;
use crate::docdata::*;

pub fn produce_base_cmd(
    origin: &[u8],
    last: &mut usize,
    current: &mut usize,
    vars: &mut HashMap<String, Vec<u8>>,
    cache: &mut HashMap<String, DocData>,
    anon_stack: &mut Vec<Vec<u8>>,
) -> Result<(), std::io::Error> {
    let basename: String = get_word_or_literal(origin, last, current)?;
    
    let mut thisbase: Base = open_base(&basename)?;

    produce_base(&mut thisbase, cache, vars, anon_stack)?;

    save_base(&thisbase)?;
    
    Ok(())
}
