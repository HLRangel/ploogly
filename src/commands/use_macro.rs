use crate::interpreter_facilities::*;
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
pub fn use_macro(
    origin: &[u8],
    current: &mut usize,
    last: &mut usize,
    vars: &mut HashMap<String, Vec<u8>>,
    anon_stack: &mut Vec<Vec<u8>>,
) -> Result<Vec<u8>, std::io::Error> {
    let macro_name: String = get_worl_produce_st(origin, current, last, vars, anon_stack)?;

    if vars.contains_key(&format!("__macro+{macro_name}")) {
        // process macro here ...
    }

    // then read everything else here...
    
    return Ok(Vec::new());
}
