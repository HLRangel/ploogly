// SPDX-License-Identifier: MPL-2.0

/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use std::collections::HashMap;

#[derive(Clone)]
pub struct DocData {
    pub ctx: HashMap<String, Vec<u8>>,
    pub path: String,
    pub data: Vec<u8>
}