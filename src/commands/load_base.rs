use crate::bases::data::open_base_vec;
use crate::interpreter_facilities::*;
use super::CommandContext;

pub fn load_base(ctx: &mut CommandContext) -> Result<(), std::io::Error> {
    let arg: String = get_word_or_literal(ctx.origin, &mut ctx.last, &mut ctx.current)?;
    ctx.result.append(&mut open_base_vec(&arg)?);
    Ok(())
}
