use crate::interpreter_facilities::*;

use std::collections::HashMap;
use std::io::ErrorKind;

pub fn var(
    result: &mut Vec<u8>,
    origin: &[u8],
    last: &mut usize,
    current: &mut usize,
    vars: &HashMap<String, Vec<u8>>,
    anon_stack: &mut Vec<Vec<u8>>,
) -> Result<(), std::io::Error> {
    let arg: String = get_word_or_literal(origin, last, current)?;

    let vari: Vec<u8> = match vars.get(&arg) {
        Some(vec) => vec.clone(),
        _ => {
            if !anon_stack.is_empty() {
                anon_stack.pop().unwrap()
            } else {
                return Err(ErrorKind::InvalidInput.into());
            }
        }
    };

    result.append(&mut nl_into_br(&vari));

    return Ok(());
}
