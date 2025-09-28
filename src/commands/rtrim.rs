use crate::interpreter_facilities::*;

use std::collections::HashMap;

pub fn rtrim(
    origin: &[u8],
    result: &mut Vec<u8>,
    current: &mut usize,
    last: &mut usize,
    vars: &mut HashMap<String, Vec<u8>>,
    anon_stack: &mut Vec<Vec<u8>>,
) -> Result<(), std::io::Error> {
    if !is_eof(origin, *current) {
        let totrim: Vec<u8> = get_worl_produce(origin, current, last, vars, anon_stack)?;
	let charsno: usize = get_worl_produce_st(origin,
						 current,
						 last,
						 vars,
						 anon_stack)?.parse::<usize>().unwrap_or(0);
	
	if totrim.len() > charsno {
            result.append(&mut (totrim[0..(totrim.len() - charsno)]).to_vec());
	}
    }

    return Ok(());
}
