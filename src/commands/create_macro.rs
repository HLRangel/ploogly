use crate::interpreter_facilities::*;
use std::collections::HashMap;

pub fn create_macro(
    origin: &[u8],
    current: &mut usize,
    last: &mut usize,
    _vars: &mut HashMap<String, Vec<u8>>,
    _anon_stack: &mut Vec<Vec<u8>>,
) -> Result<Vec<u8>, std::io::Error> {
    let params: Vec<Vec<u8>> = read_params_until_nl(origin, last, current)?;
    let macro_content: Vec<u8> = get_inner(origin, last, current)?;
    
    println!("{:?} {}", params, String::from_utf8(macro_content).unwrap());
    
    return Ok(Vec::new());
}
