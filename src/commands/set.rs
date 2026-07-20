use crate::interpreter_facilities::*;
use super::CommandContext;
use std::collections::HashMap;

pub fn set(ctx: &mut CommandContext) -> Result<(), std::io::Error> {
    if !is_eof(ctx.origin, ctx.current) {
        let name: String = get_word(ctx.origin, &mut ctx.last, &mut ctx.current)?;
        let value: Vec<u8> = get_worl_produce(ctx.origin, &mut ctx.current, &mut ctx.last, ctx.vars, ctx.anon_stack)?;

        ctx.vars.insert(name.clone(), value.clone());

        println!("SET '{}' = {}", name, String::from_utf8_lossy(&value));
    }
    Ok(())
}
