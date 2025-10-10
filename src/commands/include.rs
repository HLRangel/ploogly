use crate::file::*;
use crate::interpreter_facilities::*;

use std::collections::HashMap;

pub fn include(
    result: &mut Vec<u8>,
    origin: &[u8],
    last: &mut usize,
    current: &mut usize,
    vars: &mut HashMap<String, Vec<u8>>,
    anon_stack: &mut Vec<Vec<u8>>,
) -> Result<(), std::io::Error> {
    let arg: String = get_worl_produce_st(origin, current, last, vars, anon_stack)?;

    inclusion_into_result(result, vars, anon_stack, &arg)?;

    return Ok(());
}
