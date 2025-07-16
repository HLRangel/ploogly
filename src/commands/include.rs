// SPDX-License-Identifier: MPL-2.0

/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use crate::interpreter_facilities::*;
use crate::file::*;
use crate::docdata::DocData;

use std::collections::HashMap;

pub fn include(result: &mut Vec<u8>,
            origin: &[u8],
            last: &mut usize,
            current: &mut usize,
            vars: &HashMap<String, Vec<u8>>,
            cache: &mut HashMap<String, DocData>,
            anon_stack: &mut Vec<Vec<u8>>
        ) -> Result<(), std::io::Error> {
    let arg: String = get_word_or_literal(origin, last, current)?;
    
    inclusion_into_result(result, vars, cache, anon_stack, &arg)?;

    return Ok(());
}