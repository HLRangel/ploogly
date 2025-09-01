// SPDX-License-Identifier: MPL-2.0

/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use crate::bases::data::open_base_vec;
use crate::interpreter_facilities::*;

pub fn load_base(
    result: &mut Vec<u8>,
    origin: &[u8],
    last: &mut usize,
    current: &mut usize
) -> Result<(), std::io::Error> {
    let arg: String = get_word_or_literal(origin, last, current)?;

    result.append(&mut open_base_vec(&arg)?);

    Ok(())
}
