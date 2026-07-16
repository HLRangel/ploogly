use crate::interpreter_facilities::*;

use std::collections::HashMap;

pub fn call(
    origin: &[u8],
    current: &mut usize,
    last: &mut usize,
    vars: &mut HashMap<String, Vec<u8>>,
    anon_stack: &mut Vec<Vec<u8>>,
) -> Result<(), std::io::Error> {
    if !is_eof(origin, *current) {
        get_worl_produce_st(origin, current, last, vars, anon_stack)?;
	let argvec: Vec<Vec<u8>> = get_separated_arguments(origin, last, current, vars, anon_stack)?;

	for args in argvec {
	    println!("{}", String::from_utf8(args).unwrap());
	}
        
    }
    
    return Ok(());
}
