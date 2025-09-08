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
use crate::produce::produce;
use crate::misc::path_as_relative;

use std::fs::create_dir_all;
use std::io::Write;
use std::path::PathBuf;
use std::collections::HashMap;
use std::fs::{File, exists};
use std::io::Read;

pub fn generate(
    template: &str,
    respath: &str,
    cache: &mut HashMap<String, DocData>,
    vars: &mut HashMap<String, Vec<u8>>,
    anon_stack: &mut Vec<Vec<u8>>
) -> Result<Vec<u8>, std::io::Error> {
    let mut new_vars: HashMap<String, Vec<u8>> = vars.clone();
    let mut togen: Vec<u8> = Vec::new();

    new_vars.insert("path".to_string(), respath.as_bytes().to_vec());

    File::open(&template)?.read_to_end(&mut togen)?;

    Ok(produce(&togen, &mut new_vars, cache, anon_stack)?)
}

pub fn gen_doc_from_template(
    origin: &[u8],
    last: &mut usize,
    current: &mut usize,
    cache: &mut HashMap<String, DocData>,
    vars: &mut HashMap<String, Vec<u8>>,
    anon_stack: &mut Vec<Vec<u8>>
) -> Result<(), std::io::Error> {
    let path: String = String::from_utf8(get_worl_produce(origin, current, last, vars, cache, anon_stack)?).unwrap();
    let templatepath: String = String::from_utf8(get_worl_produce(origin, current, last, vars, cache, anon_stack)?).unwrap();

    let mut res: Vec<u8> = generate(&templatepath, &path, cache, vars, anon_stack)?;

    let pathbuf: PathBuf = path_as_relative(path);

    let mut pathdir: PathBuf = pathbuf.clone();
    pathdir.pop();
    
    if !exists(&pathdir)? {
	create_dir_all(&pathdir)?;
    }

    File::create(&pathbuf)?.write_all(&mut res)?;

    Ok(())
}
