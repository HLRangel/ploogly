use crate::interpreter_facilities::*;
use super::CommandContext;

pub fn rtrim(ctx: &mut CommandContext) -> Result<(), std::io::Error> {
    if !is_eof(ctx.origin, ctx.current) {
        let totrim: Vec<u8> = get_worl_produce(ctx.origin, &mut ctx.current, &mut ctx.last, ctx.vars, ctx.anon_stack)?;
        let charsno: usize = get_worl_produce_st(ctx.origin, &mut ctx.current, &mut ctx.last, ctx.vars, ctx.anon_stack)?.parse::<usize>().unwrap_or(0);
        if totrim.len() > charsno {
            ctx.result.append(&mut (totrim[0..(totrim.len() - charsno)]).to_vec());
        }
    }
    Ok(())
}
