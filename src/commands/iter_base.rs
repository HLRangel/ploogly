use crate::interpreter_facilities::*;
use crate::produce::*;
use crate::bases::data::*;
use super::CommandContext;
use std::collections::HashMap;

pub fn iterant(
    result: &mut Vec<u8>,
    base: &Base,
    toiter: &[u8],
    vars: &mut HashMap<String, Vec<u8>>,
    anon_stack: &mut Vec<Vec<u8>>,
) -> Result<(), std::io::Error> {
    for entry in &base.bases {
        let mut new_vars: HashMap<String, Vec<u8>> = vars.clone();
        let ctx: HashMap<String, Vec<u8>> = entry_into_hashmap(&entry)?;
        new_vars.extend(ctx);
        new_vars.insert("docdata".to_string(), entry_produced(entry)?);
        result.append(&mut produce(toiter, &mut new_vars, anon_stack)?);
    }
    Ok(())
}

pub fn iter_base(ctx: &mut CommandContext) -> Result<(), std::io::Error> {
    let mut base: Base = base_from_json(&get_worl_produce(ctx.origin, &mut ctx.current, &mut ctx.last, ctx.vars, ctx.anon_stack)?)?;
    let inner: Vec<u8> = get_inner(ctx.origin, &mut ctx.last, &mut ctx.current)?;
    iterant(ctx.result, &mut base, &inner, ctx.vars, ctx.anon_stack)?;
    Ok(())
}
