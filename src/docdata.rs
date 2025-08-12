// SPDX-License-Identifier: MPL-2.0

/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use crate::misc::*;

use std::collections::HashMap;
use std::hash::{Hasher, Hash};

#[derive(Clone)]
pub struct DocData {
    pub ctx: HashMap<String, Vec<u8>>,
    pub path: String,
    pub data: Vec<u8>,
}


impl Hash for DocData {
    fn hash<H: Hasher>(&self, state: &mut H) {
        hash_hashmap(&self.ctx).hash(state);
        self.path.hash(state);
        self.data.hash(state);
    }
}