use crate::interpreter_facilities::*;
use crate::produce::*;
use super::CommandContext;
use std::collections::HashMap;
use std::fs::*;
use std::io::ErrorKind;

fn path_prod(
    vars: &HashMap<String, Vec<u8>>,
    anon_stack: &mut Vec<Vec<u8>>,
    path: &str,
    inner: &[u8],
    result: &mut Vec<u8>,
) -> Result<(), std::io::Error> {
    let pathdata: Metadata = metadata(path)?;
    if pathdata.is_dir() {
        for file in read_dir(path)? {
            let name: String = file?.file_name().into_string().unwrap();
            let newpath: String = format!("{path}/{name}");
            let data: Metadata = metadata(&newpath)?;
            if data.is_dir() {
                path_prod(vars, anon_stack, &newpath, inner, result)?;
            } else if data.is_file() {
                let mut new_vars: HashMap<String, Vec<u8>> = vars.clone();
                new_vars.insert("filepath".to_string(), newpath.as_bytes().to_vec());
                result.append(&mut produce(inner, &mut new_vars, anon_stack)?);
            }
        }
        return Ok(());
    }
    Err(ErrorKind::NotADirectory.into())
}

pub fn iter_dir(ctx: &mut CommandContext) -> Result<Vec<u8>, std::io::Error> {
    let path: String = String::from_utf8(get_worl_produce(ctx.origin, &mut ctx.current, &mut ctx.last, ctx.vars, ctx.anon_stack)?).unwrap();
    let inner: Vec<u8> = get_inner(ctx.origin, &mut ctx.last, &mut ctx.current)?;
    let mut result: Vec<u8> = Vec::new();
    path_prod(ctx.vars, ctx.anon_stack, &path, &inner, &mut result)?;
    Ok(result)
}
