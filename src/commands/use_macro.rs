use crate::interpreter_facilities::*;
use std::collections::HashMap;

/* How macros work in Ploogly:
    PLG!M\n<macro name>\n<positional argument 1 name>...PLG!M

    ![ key_name ]! gets replaced by key;
*/

struct MacroData {
    name: String,
    positions: Vec<String>
}

fn get_macro_data(data: &[u8]) -> Result<MacroData, std::io::Error> {

    
    return Ok(MacroData {
        name: (), 
        positions: () 
    })
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
