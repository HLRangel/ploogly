use crate::docdata::DocData;
use crate::file::inclusion_into_result;
use crate::misc::hash_hashmap;
use crate::produce::produce;

use std::collections::HashMap;
use std::fs::{exists, metadata, Metadata};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io::ErrorKind;

/* NOTA BENE!

    Still need a way to store KV state so we don't have to re-produce the doc...
*/

struct ProdInfo {
    hash:   u64,
    data:   Vec<u8>
}

enum BaseData {
    Abstract,
    Produced(ProdInfo)
}

struct BaseEntry {
    id: u64,
    path: String,
    data: BaseData
}

struct Base {
    tallest: u64,
    bases: Vec<BaseEntry>
}

fn base_to_file(base: &Base, path: &str) {

}

fn base_from_file(path: &str) -> Base {

}

fn hash_state(
    cache: &mut HashMap<String, DocData>,
    vars: &mut HashMap<String, Vec<u8>>,
    anon_stack: &mut Vec<Vec<u8>>
) -> u64 {
    let mut h = DefaultHasher::new();
    anon_stack.hash(&mut h);

    hash_hashmap(vars) ^ hash_hashmap(cache) ^ h.finish()
}

fn produce_base(
    base: &mut Base,
    cache: &mut HashMap<String, DocData>,
    vars: &mut HashMap<String, Vec<u8>>,
    anon_stack: &mut Vec<Vec<u8>>
) -> Result<(), std::io::Error> {
    for i in 0..base.bases.len() {
        let mut result: Vec<u8> = Vec::new();
        
        let data: BaseData = match inclusion_into_result(
                                    &mut result,
                                    vars, 
                                    cache, 
                                    anon_stack, 
                                    &base.bases[i].path
        ) {
            Ok(_) => {
                BaseData::Abstract
            },

            Err(_) => {
                let mut h: DefaultHasher = DefaultHasher::new();
                result.hash(&mut h);


                BaseData::Produced(                
                    ProdInfo {
                        data: result,
                        hash: hash_state(cache, vars, anon_stack) ^ h.finish()
                    }
                )
            }
        };

        base.bases[i].data = data;
    }

    Ok(())
}

fn base_add(base: &mut Base, path: &str) -> Result<(), std::io::Error> {
    if exists(path)? {
        let mdata: Metadata = metadata(path)?;

        if !mdata.is_dir() {
            base.bases.push(
                BaseEntry {
                    id: base.tallest,
                    path: path.to_string(),
                    data: BaseData::Abstract
                }
            );
        } else {
            return Err(ErrorKind::InvalidInput.into());
        }
    }

    Ok(())
}