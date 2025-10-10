use crate::interpreter_facilities::*;
use crate::bases::data::*;

use std::collections::HashMap;

pub fn sortbase_by_key(
    origin: &[u8],
    result: &mut Vec<u8>,
    current: &mut usize,
    last: &mut usize,
    vars: &mut HashMap<String, Vec<u8>>,
    anon_stack: &mut Vec<Vec<u8>>,
) -> Result<(), std::io::Error> {
    if !is_eof(origin, *current) {
	let ext: String = get_word_or_literal(origin, last, current)?;
        let mut base: Base = base_from_json(&get_worl_produce(origin, current, last, vars, anon_stack)?)?;

	base_sort_by_key(&mut base, &ext)?;

	result.append(&mut base.to_json()?);
    }

    return Ok(());
}
