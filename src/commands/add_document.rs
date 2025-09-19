// SPDX-License-Identifier: MPL-2.0

/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/
use std::collections::HashMap;

use crate::interpreter_facilities::*;
use crate::bases::data::*;

pub fn add_document(
    origin: &[u8],
    last: &mut usize,
    current: &mut usize,
    vars: &mut HashMap<String, Vec<u8>>,
    anon_stack: &mut Vec<Vec<u8>>,
) -> Result<(), std::io::Error> {
    let docpath: String = get_worl_produce_st(origin, current, last, vars, anon_stack)?;    
    let basename: String = get_word_or_literal(origin, last, current)?;
    
    let mut thisbase: Base = open_base(&basename)?;

    base_add(&mut thisbase, &docpath)?;

    save_base(&thisbase)?;
    
    Ok(())
}
