// SPDX-License-Identifier: MPL-2.0

/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use crate::docdata::*;
use crate::interpreter_facilities::*;
use crate::md2html::*;
use crate::produce::*;
use crate::var_imports::*;

use std::collections::HashMap;
use std::fs::*;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io::ErrorKind;
use std::io::{Read, Write};
use std::str::*;

fn docdata_get(
    path: &str,
    cache: &mut HashMap<String, DocData>,
) -> Result<DocData, std::io::Error> {
    let data: DocData = match cache.get(path) {
        Some(entry) => entry.clone(),

        _ => {
            let mut current: usize = 0;
            let mut last: usize = 0;

            let mut fdata: Vec<u8> = Vec::new();
            File::open(path)?.read_to_end(&mut fdata)?;
            fdata = to_normalized_vec(&fdata);

            let mut hasher: DefaultHasher = DefaultHasher::new();

            path.hash(&mut hasher);

            let new_filename: String = format!("{}.html", hasher.finish());

            let tdata: DocData = DocData {
                ctx: get_frontmatter_ctx(&fdata, &mut last, &mut current)?,
                data: to_md(&get_data_to_end(&fdata, &mut last, &mut current))?,
                path: new_filename.clone(),
            };

            cache.insert(path.to_string(), tdata.clone());

            tdata
        }
    };

    Ok(data)
}

fn doc_create_whole_ctx(
    locals: &HashMap<String, Vec<u8>>,
    globals: &HashMap<String, Vec<u8>>,
    filename: &str,
    doc: &[u8],
) -> HashMap<String, Vec<u8>> {
    let mut map: HashMap<String, Vec<u8>> = globals.clone();
    map.extend(locals.clone());

    map.insert(
        "path".to_string(),
        format!("posts/{}", filename).as_bytes().to_vec(),
    );

    if !map.contains_key("docdata") {
        map.insert("docdata".to_string(), doc.to_vec().clone());
    }

    map
}

fn doc_place_ret_ctx(
    path: &str,
    tpath: &str,
    cache: &mut HashMap<String, DocData>,
    globals: &HashMap<String, Vec<u8>>,
    thislist: &mut Vec<HashMap<String, Vec<u8>>>,
    anon_stack: &mut Vec<Vec<u8>>,
) -> Result<HashMap<String, Vec<u8>>, std::io::Error> {
    let was_cached: bool = cache.contains_key(path);
    let data: DocData = docdata_get(path, cache)?;

    let mut map_for_template: HashMap<String, Vec<u8>> =
        doc_create_whole_ctx(&data.ctx, globals, &data.path, &data.data);

    thislist.push(map_for_template.clone());

    if !was_cached {
        if !exists("./out/site/posts/")? {
            create_dir("./out/site/posts/")?;
        }

        let mut to_produce: Vec<u8> = Vec::new();
        File::open(tpath)?.read_to_end(&mut to_produce)?;

        let mut new_file: File = File::create(format!("./out/site/posts/{}", &data.path))?;

        new_file.write_all(&produce(
            &to_produce,
            &mut map_for_template,
            cache,
            anon_stack,
        )?)?;
    }

    Ok(map_for_template)
}

fn ctxvec_to_sorted_ctxvec(
    vec: &Vec<HashMap<String, Vec<u8>>>,
    var_to: &str,
) -> Vec<HashMap<String, Vec<u8>>> {
    let mut toret: Vec<HashMap<String, Vec<u8>>> = vec.clone();
    toret.sort_by(|a, b| {
        a.get(var_to)
            .unwrap_or(&"0".as_bytes().to_vec())
            .cmp(b.get(var_to).unwrap_or(&"0".as_bytes().to_vec()))
    });

    toret
}

fn doc(
    origin: &[u8],
    last: &mut usize,
    current: &mut usize,
    cache: &mut HashMap<String, DocData>,
    globals: &HashMap<String, Vec<u8>>,
    anon_stack: &mut Vec<Vec<u8>>,
) -> Result<Vec<u8>, std::io::Error> {
    let doc_path: String = get_word_or_literal(origin, last, current)?;
    let template_path: String = get_word_or_literal(origin, last, current)?;

    if !is_eof(origin, *current) && exists(&doc_path)? && exists(&template_path)? {
        let mut ctx: HashMap<String, Vec<u8>> = doc_place_ret_ctx(
            &doc_path,
            &template_path,
            cache,
            globals,
            &mut Vec::new(),
            anon_stack,
        )?;

        let toplace: Vec<u8> = get_inner(origin, last, current)?;

        return produce(&toplace, &mut ctx, cache, anon_stack);
    }

    Err(ErrorKind::InvalidInput.into())
}

fn iter_doc(
    origin: &[u8],
    path: &str,
    template_path: &str,
    sortby: &str,
    order: &str,
    cache: &mut HashMap<String, DocData>,
    globals: &HashMap<String, Vec<u8>>,
    anon_stack: &mut Vec<Vec<u8>>,
) -> Result<Vec<u8>, std::io::Error> {
    let mut result: Vec<u8> = Vec::new();

    let mut ctxlist: Vec<HashMap<String, Vec<u8>>> = Vec::new();
    for dire in read_dir(path)? {
        let ipath: String = String::from_str(dire?.path().to_str().unwrap()).unwrap();
        if metadata(&ipath)?.is_file() && ipath.ends_with(".md") {
            doc_place_ret_ctx(
                &ipath,
                template_path,
                cache,
                globals,
                &mut ctxlist,
                anon_stack,
            )?;
        }
    }

    ctxlist = match order {
        "normal" => ctxvec_to_sorted_ctxvec(&ctxlist, sortby),
        "reverse" => {
            let mut vec = ctxvec_to_sorted_ctxvec(&ctxlist, sortby);
            vec.reverse();

            vec.clone()
        }
        _ => return Err(ErrorKind::InvalidInput.into()),
    };

    for entry in ctxlist {
        let mut thisentry: HashMap<String, Vec<u8>> = entry.clone();
        result.append(&mut produce(origin, &mut thisentry, cache, anon_stack)?);
    }

    Ok(result)
}

fn docs_in(
    origin: &[u8],
    last: &mut usize,
    current: &mut usize,
    cache: &mut HashMap<String, DocData>,
    globals: &HashMap<String, Vec<u8>>,
    anon_stack: &mut Vec<Vec<u8>>,
) -> Result<Vec<u8>, std::io::Error> {
    let folder_path: String = get_word_or_literal(origin, last, current)?;
    let template_path: String = get_word_or_literal(origin, last, current)?;
    let sortby: String = get_word_or_literal(origin, last, current)?;
    let order: String = get_word_or_literal(origin, last, current)?;

    let inner: Vec<u8> = get_inner(origin, last, current)?;

    if !is_eof(origin, *current)
        && exists(&folder_path)?
        && metadata(&folder_path)?.is_dir()
        && exists(&template_path)?
    {
        return Ok(iter_doc(
            &inner,
            &folder_path,
            &template_path,
            &sortby,
            &order,
            cache,
            globals,
            anon_stack,
        )?);
    }

    return Err(ErrorKind::InvalidInput.into());
}

pub fn list_doc(
    origin: &[u8],
    last: &mut usize,
    current: &mut usize,
    cache: &mut HashMap<String, DocData>,
    globals: &HashMap<String, Vec<u8>>,
    anon_stack: &mut Vec<Vec<u8>>,
) -> Result<Vec<u8>, std::io::Error> {
    let opt: String = get_word(origin, last, current)?;

    match opt.as_str() {
        "doc" => {
            return Ok(doc(origin, last, current, cache, globals, anon_stack)?);
        }

        "docs_in" => {
            return Ok(docs_in(origin, last, current, cache, globals, anon_stack)?);
        }

        _ => {
            return Err(ErrorKind::InvalidInput.into());
        }
    }
}
