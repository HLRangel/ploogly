use crate::interpreter_facilities::*;
use crate::bases::data::*;
use super::CommandContext;

pub fn add_document(ctx: &mut CommandContext) -> Result<(), std::io::Error> {
    let docpath: String = get_worl_produce_st(ctx.origin, &mut ctx.current, &mut ctx.last, ctx.vars, ctx.anon_stack)?;
    let basename: String = get_word_or_literal(ctx.origin, &mut ctx.last, &mut ctx.current)?;
    let mut thisbase: Base = open_base(&basename)?;
    base_add(&mut thisbase, &docpath)?;
    save_base(&thisbase)?;
    Ok(())
}
