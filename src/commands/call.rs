// SPDX-License-Identifier: MPL-2.0

/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use crate::interpreter_facilities::*;

use std::collections::HashMap;
use std::fs::{File, exists, remove_file, OpenOptions};
use std::io::{ErrorKind, Read, Write};
use mlua::{Function, IntoLua, Lua};
use mlua::prelude::LuaError;
use mlua::Table;
use mlua::Chunk;

fn pappend(toap: &str) -> Result<(), std::io::Error> {
    let mut file: File = OpenOptions::new()
                            .write(true)
                            .append(true)
                            .open(".ploogly-lua-temp-stdout~")?;
    
    file.write(toap.as_bytes())?;
    
    Ok(())
}

fn ploogly_functions(ctx: &Lua) -> Result<Table, LuaError> {
    let functions: Table = ctx.create_table()?;
    
    let pappendl: Function = ctx.create_function(
    |ctx: &Lua, a1: String| {
        match pappend(&a1) { Ok(_) => true, Err(_) => false}.into_lua(ctx)
    })?;

    functions.set("pappend", pappendl)?;

    Ok(functions)
}

fn ploogly_argv(ctx: &Lua, args: &Vec<Vec<u8>>) -> Result<Table, LuaError> {
    let globals: Table = ctx.create_table()?;
    
    for a in 0..args.len() {
        globals.set(a + 1, 
            String::from_utf8(args[a].clone()).unwrap())?;
    }

    Ok(globals)
}

fn ploogly_master(ctx: &Lua,
                args: &Vec<Vec<u8>>
        ) -> Result<Table, LuaError> {
    let ploogly: Table = ctx.create_table()?;
    
    let functions: Table = ploogly_functions(ctx)?;
    let globals: Table = ploogly_argv(ctx, args)?;

    ploogly.set("func", functions)?;
    ploogly.set("argv", globals)?;

    Ok(ploogly)
}

pub fn call(origin: &[u8],
        last: &mut usize,
        current: &mut usize,
        vars: &HashMap<String, Vec<u8>>,
        anon_stack: &mut Vec<Vec<u8>>
    ) -> Result<Vec<u8>, std::io::Error> {

    let mut result: Vec<u8> = Vec::new();
    let file_name: String = get_word_or_literal(origin, last, current)?;
    let args: Vec<Vec<u8>> = get_separated_arguments(origin, last, current)?;

    if exists(&file_name)? {
        let mut script: String = String::new();
        File::open(&file_name)?.read_to_string(&mut script)?;

        let ctx: Lua = Lua::new();
        let code: Chunk<'_> = ctx.load(&script);

        //for (k, v) in vars {
        //    ctx.globals().set(k.clone(), 
        //        String::from_utf8(v.to_vec()).unwrap());
        //}

        File::create(".ploogly-lua-temp-stdout~")?;

            let functions: Table = ploogly_master(&ctx, &args).unwrap();

            ctx.globals().set("ploogly", functions);

            code.exec();

            File::open(".ploogly-lua-temp-stdout~")?.read_to_end(&mut result)?;

        remove_file(".ploogly-lua-temp-stdout~")?;
    }

    return Ok(result);
}