use std::collections::HashMap;

use crate::interpreter_facilities::*;
use crate::bases::data::*;

pub fn produce_base_cmd(
    origin: &[u8],
    last: &mut usize,
    current: &mut usize,
    vars: &mut HashMap<String, Vec<u8>>,
    anon_stack: &mut Vec<Vec<u8>>,
) -> Result<(), std::io::Error> {
    let basename: String = get_word_or_literal(origin, last, current)?;
    
    let mut thisbase: Base = open_base(&basename)?;

    produce_base(&mut thisbase, vars, anon_stack)?;

    save_base(&thisbase)?;
    
    Ok(())
}
