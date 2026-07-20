use crate::file::inclusion_into_result;
use crate::interpreter_facilities::*;
use crate::var_imports::*;
use super::CommandContext;
use std::collections::HashMap;

fn interpret_var_or_stack_push(
    vec: &Vec<Vec<u8>>,
    vars: &mut HashMap<String, Vec<u8>>,
    anon_stack: &mut Vec<Vec<u8>>,
) -> Result<(), std::io::Error> {
    for arg in vec.iter().rev() {
        if arg.contains(&b':') {
            vars.extend(import_variables(&arg)?);
        } else {
            anon_stack.push(arg.to_vec());
        }
    }
    Ok(())
}

pub fn template(ctx: &mut CommandContext) -> Result<Vec<u8>, std::io::Error> {
    let filename: String = get_word_or_literal(ctx.origin, &mut ctx.last, &mut ctx.current)?;
    let vec: Vec<Vec<u8>> = get_separated_arguments(ctx.origin, &mut ctx.last, &mut ctx.current, ctx.vars, ctx.anon_stack)?;

    let mut thisstack: Vec<Vec<u8>> = ctx.anon_stack.clone();
    let mut thisvars: HashMap<String, Vec<u8>> = ctx.vars.clone();
    let mut result: Vec<u8> = Vec::new();

    interpret_var_or_stack_push(&vec, &mut thisvars, &mut thisstack)?;

    inclusion_into_result(&mut result, &thisvars, &mut thisstack, &filename)?;

    Ok(result)
}
