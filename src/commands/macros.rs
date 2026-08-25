use crate::{interpreter_facilities::*, produce::produce};
use super::CommandContext;
use std::io::ErrorKind;

/* How macros work in Ploogly:
    PG!M<macro name>\n<positional argument 1 name>...\nPG!E

    ![ key_name ]! gets replaced by key;
*/

struct MacroData {
    name: String,
    positions: Vec<String>,
    code: String
}

fn get_macro_data(data: &[u8]) -> Result<MacroData, std::io::Error> {
    let mut current: usize = 0;
    let mut last: usize = 0;

    if data.starts_with(&[b'P', b'G', b'!', b'M']) {
        current += 4;
        last = current;

        let mut macrodata: MacroData = MacroData { 
            name: "null".to_string(), 
            positions: Vec::new(), 
            code: String::new() 
        };

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
            // Return an "empty macro" text if there's no terminator.
            if is_eof(data, current) {
                macrodata.code = "empty macro".to_string();
            } else {
                macrodata.code = String::from_utf8(data[current + 4..].to_vec()).unwrap();
            }

            macrodata.name = macrodata.positions[0].clone();

            return Ok(macrodata)
        }
    }

    Err(ErrorKind::InvalidData.into())
}

pub fn use_macro(ctx: &mut CommandContext) -> Result<Vec<u8>, std::io::Error> {
    let macro_name: String = get_worl_produce_st(ctx.origin, &mut ctx.current, &mut ctx.last, ctx.vars, ctx.anon_stack)?;

    if ctx.vars.contains_key(&format!("__macro+{macro_name}")) {
        let mut macrodata: MacroData = get_macro_data(ctx.vars.get(&format!("__macro+{macro_name}")).unwrap())?;
        let args: Vec<String> = get_separated_arguments_st(ctx.origin, &mut ctx.last, &mut ctx.current, ctx.vars, ctx.anon_stack)?;
        
        if args.len() > 1 {
            let mut index: usize = 0;

            for argument in &macrodata.positions[1..] {
                macrodata.code = macrodata.code.replace(&format!("#![ {argument} ]!#"), &args[index]);
            
                index += 1;
            }
        }

        return Ok(produce(macrodata.code.as_bytes(), ctx.vars, ctx.anon_stack)?);
    }

    return Err(ErrorKind::InvalidData.into());
}

pub fn create_macro(ctx: &mut CommandContext) -> Result<(), std::io::Error> {
    let macro_name: String = get_worl_produce_st(ctx.origin, &mut ctx.current, &mut ctx.last, ctx.vars, ctx.anon_stack)?;
    let macro_args: Vec<String> = get_separated_arguments_st(ctx.origin, &mut ctx.last, &mut ctx.current, &mut ctx.vars, &mut ctx.anon_stack)?;
    
    let macro_content: String = get_inner_st(ctx.origin, &mut ctx.last, &mut ctx.current)?;

    ctx.vars.insert(format!("__macro+{macro_name}"), macro_content.clone());
    
    Ok(())
}
