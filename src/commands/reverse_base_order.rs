use crate::interpreter_facilities::*;
use crate::bases::data::*;
use super::CommandContext;

pub fn reverse_base_order(ctx: &mut CommandContext) -> Result<(), std::io::Error> {
    if !is_eof(ctx.origin, ctx.current) {
        let mut base: Base = base_from_json(&get_worl_produce(ctx.origin, &mut ctx.current, &mut ctx.last, ctx.vars, ctx.anon_stack)?)?;
        base.bases.reverse();
        ctx.result.append(&mut base.to_json()?);
    }
    Ok(())
}
