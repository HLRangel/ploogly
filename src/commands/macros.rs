use crate::interpreter_facilities::*;
use super::CommandContext;

pub fn create_macro(ctx: &mut CommandContext) -> Result<(), std::io::Error> {
    let params: Vec<Vec<u8>> = read_params_until_nl(ctx.origin, &mut ctx.last, &mut ctx.current)?;
    let macro_content: Vec<u8> = get_inner(ctx.origin, &mut ctx.last, &mut ctx.current)?;
    
    println!("{:?} {}", params, String::from_utf8(macro_content).unwrap());
    
    Ok(())
}
