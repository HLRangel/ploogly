use crate::interpreter_facilities::*;

use std::collections::HashMap;

pub fn set(
    origin: &[u8],
    current: &mut usize,
    last: &mut usize,
    vars: &mut HashMap<String, Vec<u8>>,
    anon_stack: &mut Vec<Vec<u8>>,
) -> Result<(), std::io::Error> {
    if !is_eof(origin, *current) {
        let name: String = get_word(origin, last, current)?;
        let value: Vec<u8> = get_worl_produce(origin, current, last, vars, anon_stack)?;

        vars.insert(name, value);
    }

    return Ok(());
}
