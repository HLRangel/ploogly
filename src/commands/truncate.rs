use crate::interpreter_facilities::*;
use crate::produce::*;
use super::CommandContext;
use std::io::ErrorKind;

pub fn truncate(ctx: &mut CommandContext) -> Result<Vec<u8>, std::io::Error> {
    let to_truncate: String = get_word_or_literal(ctx.origin, &mut ctx.last, &mut ctx.current)?;
    let num: usize = match get_word_or_literal(ctx.origin, &mut ctx.last, &mut ctx.current)?.parse() {
        Err(_) => return Err(ErrorKind::InvalidInput.into()),
        Ok(no) => no,
    };

    let mut result: String =
        String::from_utf8(produce(to_truncate.as_bytes(), ctx.vars, ctx.anon_stack)?).unwrap();
    if result.chars().count() > num {
        result = result.chars().take(num - 1).collect();
        result.push_str("...");
    }

    Ok(result.as_bytes().to_vec())
}
