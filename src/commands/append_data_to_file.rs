use crate::interpreter_facilities::*;
use crate::produce::produce;
use crate::misc::path_as_relative;

use std::collections::HashMap;
use std::fs::{create_dir_all, OpenOptions, exists};
use std::io::Write;
use std::path::PathBuf;

/// Appends the processed inner content to the named file
/// inside the site output directory.
///
/// Usage: `{{ append_data_to_file <filename> <inner> }}`
pub fn append_data_to_file(
    origin: &[u8],
    last: &mut usize,
    current: &mut usize,
    vars: &mut HashMap<String, Vec<u8>>,
    anon_stack: &mut Vec<Vec<u8>>,
) -> Result<(), std::io::Error> {
    let filename: String = get_worl_produce_st(origin, current, last, vars, anon_stack)?;
    let inner: Vec<u8> = get_inner(origin, last, current)?;

    // Normalize CR-LF to LF before processing
    let inner = to_normalized_vec(&inner);

    let processed: Vec<u8> = produce(&inner, vars, anon_stack)?;

    let out_path: PathBuf = path_as_relative(filename);

    // Ensure the parent directory exists
    if let Some(parent) = out_path.parent() {
        if !exists(parent)? {
            create_dir_all(parent)?;
        }
    }

    let mut file = OpenOptions::new().append(true).create(true).open(out_path)?;
    file.write_all(&processed)?;

    Ok(())
}
