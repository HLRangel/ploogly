use crate::file::*;
use crate::interpreter_facilities::*;
use super::CommandContext;

pub fn include(ctx: &mut CommandContext) -> Result<(), std::io::Error> {
    let arg: String = get_worl_produce_st(ctx.origin, &mut ctx.current, &mut ctx.last, ctx.vars, ctx.anon_stack)?;

    inclusion_into_result(ctx.result, &*ctx.vars, ctx.anon_stack, &arg)?;

    Ok(())
}
