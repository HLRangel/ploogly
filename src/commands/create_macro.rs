use crate::interpreter_facilities::*;
use std::collections::HashMap;

/* How macros work in Ploogly:

    ![ key_name ]! gets replaced by key;
*/

// Just pushes inner content as a variable
pub fn create_macro(
    origin: &[u8],
    current: &mut usize,
    last: &mut usize,
    vars: &mut HashMap<String, Vec<u8>>,
    anon_stack: &mut Vec<Vec<u8>>,
) -> Result<Vec<u8>, std::io::Error> {
    let macro_name: String = get_worl_produce_st(origin, current, last, vars, anon_stack)?;
    let macro_content: Vec<u8> = get_inner(origin, last, current)?;

    vars.insert(format!("__macro+{macro_name}"), macro_content);
    
    return Ok(Vec::new());
}
