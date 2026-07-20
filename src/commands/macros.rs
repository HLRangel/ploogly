use crate::interpreter_facilities::*;
use super::CommandContext;
use std::{collections::HashMap, io::ErrorKind};

/* How macros work in Ploogly:
    PG!M<macro name>\n<positional argument 1 name>...\nPG!E

    ![ key_name ]! gets replaced by key;
*/

struct MacroData {
    name: String,
    positions: Vec<String>
}

fn get_macro_data(data: &[u8]) -> Result<MacroData, std::io::Error> {
    let mut current: usize = 0;
    let mut last: usize = 0;

    if data.starts_with(&[b'P', b'G', b'!', b'M']) {
        current += 4;
        last = current;

        let mut macrodata: MacroData = MacroData { name: "null".to_string(), positions: Vec::new() };
        while !data[current..].starts_with(&[b'P', b'G', b'!', b'E']) {
            while data[current] != b'\n' && !is_eof(data, current) {
                current += 1;
            }

            macrodata.positions.push(String::from_utf8(data[last..current - 1].to_vec()).unwrap());
            
            if !is_eof(data, current) {
                last = current + 1;
                current += 1;
            }
        }

        if macrodata.positions.len() == 0 {
            return Err(ErrorKind::InvalidData.into())
        } else {
            macrodata.name = macrodata.positions[0].clone();

            return Ok(macrodata)
        }
    }

    Err(ErrorKind::InvalidData.into())
}

// Just pushes inner content as a variable
pub fn use_macro(ctx: &mut CommandContext) -> Result<Vec<u8>, std::io::Error> {
    let macro_name: String = get_worl_produce_st(ctx.origin, &mut ctx.current, &mut ctx.last, ctx.vars, ctx.anon_stack)?;

    if ctx.vars.contains_key(&format!("__macro+{macro_name}")) {
        let macrodata: MacroData = get_macro_data(ctx.vars.get(&format!("__macro+{macro_name}")).unwrap())?;
        let args: Vec<Vec<u8>> = get_separated_arguments(ctx.origin, &mut ctx.last, &mut ctx.current, ctx.vars, ctx.anon_stack)?;
    
        // TODO: actual macro expansion
    }

    return Err(ErrorKind::InvalidData.into());
}

pub fn create_macro(ctx: &mut CommandContext) -> Result<(), std::io::Error> {
    let macro_name: String = get_worl_produce_st(ctx.origin, &mut ctx.current, &mut ctx.last, ctx.vars, ctx.anon_stack)?;
    let macro_content: Vec<u8> = get_inner(ctx.origin, &mut ctx.last, &mut ctx.current)?;

    ctx.vars.insert(format!("__macro+{macro_name}"), macro_content.clone());

    debug_println!("MACRO '{}' registered", macro_name);
    
    Ok(())
}
