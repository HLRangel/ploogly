use std::collections::HashMap;
use crate::interpreter_facilities::*;
use crate::bases::data::*;
use super::CommandContext;

pub fn produce_base_cmd(ctx: &mut CommandContext) -> Result<(), std::io::Error> {
    let basename: String = get_word_or_literal(ctx.origin, &mut ctx.last, &mut ctx.current)?;
    let mut thisbase: Base = open_base(&basename)?;
    produce_base(&mut thisbase, ctx.vars, ctx.anon_stack)?;
    save_base(&thisbase)?;
    Ok(())
}
