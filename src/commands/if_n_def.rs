use std::collections::HashMap;

use crate::interpreter_facilities::{get_inner, get_word_or_literal};
use crate::produce::*;

pub fn ifdef(
    origin: &[u8],
    current: &mut usize,
    last: &mut usize,
    vars: &mut HashMap<String, Vec<u8>>,
    anon_stack: &mut Vec<Vec<u8>>,
) -> Result<Vec<u8>, std::io::Error> {
    let arg: String = get_word_or_literal(origin, last, current)?;
    let inner: Vec<u8> = get_inner(origin, last, current)?;

    if vars.contains_key(&arg) {
        //let mut new: HashMap<String, Vec<u8>> = vars.clone();
        return Ok(produce(&inner, vars, anon_stack)?.to_vec());
    }

    return Ok(Vec::new());
}

pub fn ifndef(
    origin: &[u8],
    current: &mut usize,
    last: &mut usize,
    vars: &mut HashMap<String, Vec<u8>>,
    anon_stack: &mut Vec<Vec<u8>>,
) -> Result<Vec<u8>, std::io::Error> {
    let arg: String = get_word_or_literal(origin, last, current)?;
    let inner: Vec<u8> = get_inner(origin, last, current)?;

    if !vars.contains_key(&arg) {
        //let mut new: HashMap<String, Vec<u8>> = vars.clone();
        return Ok(produce(&inner, vars, anon_stack)?.to_vec());
    }

    return Ok(Vec::new());
}
