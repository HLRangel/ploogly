use crate::interpreter_facilities::*;
use crate::produce::produce;
use crate::misc::path_as_relative;
use super::CommandContext;
use std::fs::{create_dir_all, OpenOptions, exists};
use std::io::Write;
use std::path::PathBuf;

pub fn append_data_to_file(ctx: &mut CommandContext) -> Result<(), std::io::Error> {
    let filename: String = get_worl_produce_st(ctx.origin, &mut ctx.current, &mut ctx.last, ctx.vars, ctx.anon_stack)?;
    let inner: Vec<u8> = get_inner(ctx.origin, &mut ctx.last, &mut ctx.current)?;
    let inner = to_normalized_vec(&inner);
    let processed: Vec<u8> = produce(&inner, ctx.vars, ctx.anon_stack)?;
    let out_path: PathBuf = path_as_relative(filename);
    if let Some(parent) = out_path.parent() {
        if !exists(parent)? {
            create_dir_all(parent)?;
        }
    }
    let mut file = OpenOptions::new().append(true).create(true).open(out_path)?;
    file.write_all(&processed)?;
    Ok(())
}
