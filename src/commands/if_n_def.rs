use crate::interpreter_facilities::{get_inner, get_word_or_literal};
use crate::produce::*;
use super::CommandContext;

pub fn ifdef(ctx: &mut CommandContext) -> Result<Vec<u8>, std::io::Error> {
    let arg: String = get_word_or_literal(ctx.origin, &mut ctx.last, &mut ctx.current)?;
    let inner: Vec<u8> = get_inner(ctx.origin, &mut ctx.last, &mut ctx.current)?;

    if ctx.vars.contains_key(&arg) {
        return Ok(produce(&inner, ctx.vars, ctx.anon_stack)?.to_vec());
    }
    Ok(Vec::new())
}

pub fn ifndef(ctx: &mut CommandContext) -> Result<Vec<u8>, std::io::Error> {
    let arg: String = get_word_or_literal(ctx.origin, &mut ctx.last, &mut ctx.current)?;
    let inner: Vec<u8> = get_inner(ctx.origin, &mut ctx.last, &mut ctx.current)?;

    if !ctx.vars.contains_key(&arg) {
        return Ok(produce(&inner, ctx.vars, ctx.anon_stack)?.to_vec());
    }
    Ok(Vec::new())
}
