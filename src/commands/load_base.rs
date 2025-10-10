use crate::bases::data::open_base_vec;
use crate::interpreter_facilities::*;

pub fn load_base(
    result: &mut Vec<u8>,
    origin: &[u8],
    last: &mut usize,
    current: &mut usize
) -> Result<(), std::io::Error> {
    let arg: String = get_word_or_literal(origin, last, current)?;

    result.append(&mut open_base_vec(&arg)?);

    Ok(())
}
