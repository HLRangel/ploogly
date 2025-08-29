use crate::docdata::DocData;
use crate::file::inclusion_into_result;
use crate::md2html::*;
use crate::interpreter_facilities::get_data_to_end;

use std::collections::HashMap;
use std::fs::{exists, metadata, Metadata, File};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io::{ErrorKind, Read};

/* NOTA BENE!

    Still need a way to store KV state so we don't have to re-produce the doc...

    If it's Markdown: get frontmatter data, put into kv
    If it's HTML: find some other way, Okay
 */

/* ig, so off the top of my head, there are a few ways to go about this.

Essentially, we want the user to access the elements of the inner context
during the process of production in the produce_base iterator block. This
effectively means that we store

- The file path
- A hash of the file metadata
- The file's produced content
- The disjunction of the inner context with the outer / the frontmatter

ALSO, producing a base should not create new files. We need a new command,
e.g create_file_at, to create a file from a var. Also, introduce text
trimming functions and regex to allow the user to do procedural file
paths as they desire.

The iterator should, eventually, allow access only to docdata and the
key-value pair...
 */

enum Context {
    Absent,
    Exists(Vec<(String, Vec<u8>)>)
}

struct ProdInfo {
    oglen:  u64,
    data:   Vec<u8>,
    ctx:    Context
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

fn base_to_binvec(base: &Base) -> Result<Vec<u8>, std::io::Error> {
    let mut result: Vec<u8> = Vec::new();
    
    for entry in base {

    }

    Ok(result)
}


fn binvec_to_base(binvec: &[u8]) -> Base {

}

// Return elements from a which do not exist in b
fn disjunct_tuplevec<T: PartialEq + Clone, U: PartialEq + Clone>(
    a: &Vec<(T, U)>, b: &Vec<(T, U)>
) -> Vec<(T, U)>{
    let mut toret: Vec<(T, U)> = Vec::new();

    for tuple in a {
        let mut push: bool = true;

        for subtuple in b {
            if tuple == subtuple {
                push = false;
                break;
            }
        }

        if push {
            toret.push(tuple.clone());
        }
    }

    toret
}

fn varmap_to_tuple(
    vars: &HashMap<String, Vec<u8>>
) -> Result<Vec<(String, Vec<u8>)>, std::io::Error> {
    let mut res: Vec<(String, Vec<u8>)> = Vec::new();

    for (k, v) in vars.into_iter() {
        res.push((k.clone(), v.clone()));
    }

    Ok(res)
}

fn produce_base(
    base: &mut Base,
    cache: &mut HashMap<String, DocData>,
    vars: &mut HashMap<String, Vec<u8>>,
    anon_stack: &mut Vec<Vec<u8>>
) -> Result<(), std::io::Error> {
    for i in 0..base.bases.len() {

	// What we need here: a way to determine HTML, e.g ishtml(), ismd()
	// If we want to avoid opening the file, maybe a "parseas" override?
	if exists(&base.bases[i].path)? {
	    // get metadata
	    let file_meta: Metadata = metadata(&base.bases[i].path)?;
	    let file_len: u64 = file_meta.len();

	    let mut result: Vec<u8> = Vec::new();
	    
	    if !file_meta.is_dir() {
		// everything else
		if base.bases[i].path.ends_with(".html") || base.bases[i].path.ends_with(".htm") {
		    let vars_pre: Vec<(String, Vec<u8>)> = varmap_to_tuple(vars)?;
		    
		    inclusion_into_result(&mut result, vars, cache, anon_stack, &base.bases[i].path)?;
		    
		    let to_store: Vec<(String, Vec<u8>)> = disjunct_tuplevec(&varmap_to_tuple(vars)?, &vars_pre);

		    base.bases[i].data = BaseData::Produced(ProdInfo {
			oglen: file_len,
			data: result,
			ctx: Context::Exists(to_store.clone())
		    });
		} else if base.bases[i].path.ends_with(".md") || base.bases[i].path.ends_with(".markdown") {
		    let mut origin: Vec<u8> = Vec::new();
		    File::open(&base.bases[i].path)?.read_to_end(&mut origin)?;

		    let mut current: usize = 0;
		    let mut last: usize = 0;

		    let md_ctx: Vec<(String, Vec<u8>)> = varmap_to_tuple(
			&get_frontmatter_ctx(&origin, &mut last, &mut current)?)?;

		    result = to_md(&get_data_to_end(&origin, &mut last, &mut current))?;

		    base.bases[i].data = BaseData::Produced(ProdInfo {
			oglen: file_len,
			data: result,
			ctx: Context::Absent
		    });
		} else {
		    // Treat as plaintext
		    let mut open_file = File::open(&base.bases[i].path)?;
		    open_file.read_to_end(&mut result)?;
		    
		    base.bases[i].data = BaseData::Produced(ProdInfo {
			oglen: file_len,
			data: result,
			ctx: Context::Absent
		    });
		}
	    }
	}
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

	    base.tallest += 1;
        } else {
            return Err(ErrorKind::InvalidInput.into());
        }
    }

    Ok(())
}
