use crate::interpreter_facilities::*;
use super::CommandContext;
use std::io::ErrorKind;

pub fn var(ctx: &mut CommandContext) -> Result<(), std::io::Error> {
    let arg: String = get_word_or_literal(ctx.origin, &mut ctx.last, &mut ctx.current)?;

    let vari: Vec<u8> = match ctx.vars.get(&arg) {
        Some(vec) => vec.clone(),
        _ => {
            if !ctx.anon_stack.is_empty() {
                ctx.anon_stack.pop().unwrap()
            } else {
                return Err(ErrorKind::InvalidInput.into());
            }
        }
    };

    ctx.result.append(&mut nl_into_br(&vari));

    Ok(())
}
