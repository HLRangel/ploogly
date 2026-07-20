use crate::interpreter_facilities::*;
use super::CommandContext;
use std::collections::HashMap;

pub fn unset(ctx: &mut CommandContext) -> Result<(), std::io::Error> {
    let to_unset: String = get_word_or_literal(ctx.origin, &mut ctx.last, &mut ctx.current)?;
    if ctx.vars.contains_key(&to_unset) {
        ctx.vars.remove(&to_unset);
    }
    Ok(())
}
