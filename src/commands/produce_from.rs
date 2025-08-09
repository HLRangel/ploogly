// SPDX-License-Identifier: MPL-2.0

/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use crate::docdata::*;
use crate::interpreter_facilities::*;
use crate::produce::*;
use crate::var_imports::*;

use std::collections::HashMap;
use std::fs::*;
use std::io::ErrorKind;
use std::str::*;

fn get_context(
    origin: &[u8],
    last: &mut usize,
    current: &mut usize,
) -> Result<HashMap<String, Vec<u8>>, std::io::Error> {
    let arg: String = get_word(origin, last, current)?;

    let vars: HashMap<String, Vec<u8>> = get_vars_from_file(&format!("./contexts/{arg}"))?;

    return Ok(vars);
}

fn iter_produce(
    template: Vec<u8>,
    path: &str,
    vars: &HashMap<String, Vec<u8>>,
    cache: &mut HashMap<String, DocData>,
    anon_stack: &mut Vec<Vec<u8>>,
) -> Result<Vec<u8>, std::io::Error> {
    if exists(path)? && metadata(path)?.is_dir() {
        let mut result: Vec<u8> = Vec::new();

        for dire in read_dir(path)? {
            let ipath: String = String::from_str(dire?.path().to_str().unwrap()).unwrap();
            if metadata(&ipath)?.is_file() && ipath.ends_with(".html") {
                let mut varscp: HashMap<String, Vec<u8>> = vars.clone();
                varscp.extend(get_vars_from_file(&ipath)?);

                let mut prod: Vec<u8> = produce(&template, &mut varscp, cache, anon_stack)?;

                result.append(&mut prod);
                result.push(b'\n');
            }
        }

        return Ok(result);
    }

    return Err(ErrorKind::NotFound.into());
}

/*
    fn get_n_arguments(origin: &[u8], last: &mut usize, current: &mut usize,
        n: i32) -> Result<Vec<String>, std::io::Error> {

        if n > 0 {
            let mut svec: Vec<String> = Vec::new();
            for i in 0..n - 1{
                if !is_eof(origin, *current) {
                    svec[i as usize] = get_word_or_literal(origin, last, current);
                    if is_eof(origin, *current) {
                        return Err(ErrorKind::InvalidInput.into());
                    }

                    *current += 1;
                } else {
                    return Err(ErrorKind::InvalidInput.into());
                }
            }

            return Ok(svec);
        } else {
            return Err(ErrorKind::InvalidInput.into());
        }
    }
*/

fn contexts_in(
    origin: &[u8],
    last: &mut usize,
    current: &mut usize,
    vars: &HashMap<String, Vec<u8>>,
    cache: &mut HashMap<String, DocData>,
    anon_stack: &mut Vec<Vec<u8>>,
) -> Result<Vec<u8>, std::io::Error> {
    let toret: Vec<u8>;
    let argument: String;

    *current += 1;

    if !is_eof(origin, *current) {
        argument = get_word_or_literal(origin, last, current)?;

        let inner: Vec<u8> = get_inner(origin, last, current)?;

        toret = iter_produce(inner, &argument, vars, cache, anon_stack)?;
    } else {
        return Err(ErrorKind::InvalidInput.into());
    }

    return Ok(toret);
}

pub fn produce_from(
    origin: &[u8],
    last: &mut usize,
    current: &mut usize,
    vars: &HashMap<String, Vec<u8>>,
    cache: &mut HashMap<String, DocData>,
    anon_stack: &mut Vec<Vec<u8>>,
) -> Result<Vec<u8>, std::io::Error> {
    let com: String = get_word(origin, last, current)?;

    match com.as_str() {
        "context" => {
            let mut context: HashMap<String, Vec<u8>> = vars.clone();
            context.extend(get_context(origin, last, current)?);

            let toprod: Vec<u8> = get_inner(origin, last, current)?;

            let res: Vec<u8> = produce(&toprod, &mut context, cache, anon_stack)?;

            return Ok(res);
        }

        // create optional argument: "sortby"
        "contexts_in" => {
            let res: Vec<u8> = contexts_in(origin, last, current, vars, cache, anon_stack)?;

            return Ok(res);
        }

        _ => {
            eprintln!("Malformed on produce_from!");
        }
    };

    return Err(ErrorKind::InvalidInput.into());
}
