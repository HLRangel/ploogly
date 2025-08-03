// SPDX-License-Identifier: MPL-2.0

/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use crate::interpreter_facilities::*;

use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::fs::{File, exists, remove_file, OpenOptions};
use std::io::{ErrorKind, Read, Write};
use std::str::FromStr;
use mlua::{Function, IntoLua, Lua};
use mlua::prelude::LuaError;
use mlua::Table;
use mlua::Chunk;

fn filename(path: &str) -> String {
    let mut hasher: DefaultHasher = DefaultHasher::new();
    path.hash(&mut hasher);

    return format!(".ploogly-out-{}~", hasher.finish());
}

fn pprint(toap: &str,
            outname: String
        )-> Result<(), std::io::Error> {
    let mut file: File = OpenOptions::new()
                            .write(true)
                            .append(true)
                            .open(&outname)?;
    
    file.write(toap.as_bytes())?;
    
    Ok(())
}

fn ploogly_master(  ctx: &Lua,
                    outname: &str
        ) -> Result<Table, LuaError> {
    let ploogly: Table = ctx.create_table()?;
    
    // pprint
    let str: String = String::from_str(outname).unwrap();
    let pprintl: Function = ctx.create_function(
    move |ctx: &Lua, a1: String| {
        match pprint(&a1, str.clone()) { 
            Ok(_) => true,
            Err(_) => false
        }.into_lua(ctx)
    })?;

    ploogly.set("pprint", pprintl)?;

    Ok(ploogly)
}

pub fn pluacgi(path: &str) -> Result<Vec<u8>, std::io::Error> {
    let mut result: Vec<u8> = Vec::new();

    if exists(&path)? {
        let mut script: String = String::new();
        File::open(&path)?.read_to_string(&mut script)?;

        let ctx: Lua = Lua::new();
        let code: Chunk<'_> = ctx.load(&script);

        let mut outname: String = filename(path);
        
        File::create(&outname)?;

            let functions: Table = ploogly_master(&ctx, &outname).unwrap();

            ctx.globals().set("ploogly", functions);

            code.exec();

            File::open(&outname)?.read_to_end(&mut result)?;

        remove_file(&outname)?;
    }

    return Ok(result);
}