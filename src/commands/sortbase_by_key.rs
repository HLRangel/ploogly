use crate::interpreter_facilities::*;
use crate::bases::data::*;
use super::CommandContext;
use std::collections::HashMap;

pub fn sortbase_by_key(ctx: &mut CommandContext) -> Result<(), std::io::Error> {
    if !is_eof(ctx.origin, ctx.current) {
        let ext: String = get_word_or_literal(ctx.origin, &mut ctx.last, &mut ctx.current)?;
        let mut base: Base = base_from_json(&get_worl_produce(ctx.origin, &mut ctx.current, &mut ctx.last, ctx.vars, ctx.anon_stack)?)?;
        base_sort_by_key(&mut base, &ext)?;
        ctx.result.append(&mut base.to_json()?);
    }
    Ok(())
}
