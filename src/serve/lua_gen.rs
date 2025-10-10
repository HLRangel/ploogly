use crate::serve::data::*;

use mlua::Chunk;
use mlua::Table;
use mlua::prelude::LuaError;
use mlua::{Function, IntoLua, Lua};
use std::fs::{File, OpenOptions, exists, remove_file};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io::{Read, Write};
use std::str::FromStr;

fn filename(path: &str) -> String {
    let mut hasher: DefaultHasher = DefaultHasher::new();
    path.hash(&mut hasher);

    return format!(".ploogly-out-{}~", hasher.finish());
}

fn pprint(toap: &str, outname: String) -> Result<(), std::io::Error> {
    let mut file: File = OpenOptions::new().write(true).append(true).open(&outname)?;

    file.write(toap.as_bytes())?;

    Ok(())
}

fn make_kvdata(ctx: &Lua, key: &str, value: &str) -> Result<Table, LuaError> {
    let kvtable: Table = ctx.create_table()?;

    kvtable.set(1, key)?;
    kvtable.set(2, value)?;

    return Ok(kvtable);
}

fn put_req_info(ctx: &Lua, data: &ReqInfo) -> Result<Table, LuaError> {
    let datatable: Table = ctx.create_table()?;

    let mut i: usize = 1;
    for entry in data.entries.clone().unwrap() {
        datatable.set(i, make_kvdata(ctx, &entry.key, &entry.value)?)?;
        i += 1;
    }

    return Ok(datatable);
}

fn ploogly_master(ctx: &Lua, outname: &str, data: &ReqInfo) -> Result<Table, LuaError> {
    let ploogly: Table = ctx.create_table()?;

    // pprint
    let str: String = String::from_str(outname).unwrap();
    let pprintl: Function = ctx.create_function(move |ctx: &Lua, a1: String| {
        match pprint(&a1, str.clone()) {
            Ok(_) => true,
            Err(_) => false,
        }
        .into_lua(ctx)
    })?;

    // vars
    if !data.entries.is_none() {
        let vars: Table = put_req_info(ctx, data)?;
        ploogly.set("vars", vars)?;
    }

    ploogly.set("pprint", pprintl)?;
    ploogly.set("method", data.method.to_string())?;

    Ok(ploogly)
}

pub fn pluacgi(path: &str, data: &ReqInfo) -> Result<Vec<u8>, std::io::Error> {
    let mut result: Vec<u8> = Vec::new();

    if exists(&path)? {
        let mut script: String = String::new();
        File::open(&path)?.read_to_string(&mut script)?;

        let ctx: Lua = Lua::new();
        let code: Chunk<'_> = ctx.load(&script);

        let outname: String = filename(path);

        File::create(&outname)?;

        let functions: Table = ploogly_master(&ctx, &outname, data).unwrap();

        ctx.globals().set("ploogly", functions);

        code.exec();

        File::open(&outname)?.read_to_end(&mut result)?;

        remove_file(&outname)?;
    }

    return Ok(result);
}
