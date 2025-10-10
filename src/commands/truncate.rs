use crate::{interpreter_facilities::*, produce::*};
use std::{collections::HashMap, io::ErrorKind};

pub fn truncate(
    origin: &[u8],
    last: &mut usize,
    current: &mut usize,
    vars: &mut HashMap<String, Vec<u8>>,
    anon_stack: &mut Vec<Vec<u8>>,
) -> Result<Vec<u8>, std::io::Error> {
    let to_truncate: String = get_word_or_literal(origin, last, current)?;
    let num: usize = match get_word_or_literal(origin, last, current)?.parse() {
        Err(_) => return Err(ErrorKind::InvalidInput.into()),

        Ok(no) => no,
    };

    let mut result: String =
        String::from_utf8(produce(to_truncate.as_bytes(), vars, anon_stack)?).unwrap();
    if result.chars().count() > num {
        result = result.chars().take(num - 1).collect();
        result.push_str("...");
    }

    return Ok(result.as_bytes().to_vec());
}
