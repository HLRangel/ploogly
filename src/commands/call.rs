use crate::interpreter_facilities::*;
use super::CommandContext;
use std::collections::HashMap;

pub fn call(ctx: &mut CommandContext) -> Result<(), std::io::Error> {
    if !is_eof(ctx.origin, ctx.current) {
        get_worl_produce_st(ctx.origin, &mut ctx.current, &mut ctx.last, ctx.vars, ctx.anon_stack)?;
        let argvec: Vec<Vec<u8>> = get_separated_arguments(ctx.origin, &mut ctx.last, &mut ctx.current, ctx.vars, ctx.anon_stack)?;
        for args in argvec {
            debug_println!("{}", String::from_utf8(args).unwrap());
        }
    }
    Ok(())
}
