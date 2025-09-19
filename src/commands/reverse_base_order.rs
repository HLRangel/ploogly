use crate::interpreter_facilities::*;
use crate::bases::data::*;

use std::collections::HashMap;

pub fn reverse_base_order(
    origin: &[u8],
    result: &mut Vec<u8>,
    current: &mut usize,
    last: &mut usize,
    vars: &mut HashMap<String, Vec<u8>>,
    anon_stack: &mut Vec<Vec<u8>>,
) -> Result<(), std::io::Error> {
    if !is_eof(origin, *current) {
        let mut base: Base = base_from_json(&get_worl_produce(origin, current, last, vars, anon_stack)?)?;

	base.bases.reverse();
	
	result.append(&mut base.to_json()?);
    }

    return Ok(());
}
