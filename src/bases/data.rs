use crate::docdata::DocData;
use crate::file::inclusion_into_result;
use crate::md2html::*;
use crate::interpreter_facilities::get_data_to_end;

use std::collections::HashMap;
use std::fs::{exists, metadata, create_dir, Metadata, File};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io::{ErrorKind, Read, Write};
use serde::{Serialize, Deserialize};

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

#[derive(Clone, Serialize, Deserialize)]
enum Context {
    Absent,
    Exists(Vec<(String, Vec<u8>)>)
}

#[derive(Clone, Serialize, Deserialize)]
struct ProdInfo {
    oglen:  u64,
    data:   Vec<u8>,
    ctx:    Context
}

#[derive(Clone, Serialize, Deserialize)]
enum BaseData {
    Abstract,
    Produced(ProdInfo)
}

#[derive(Clone, Serialize, Deserialize)]
struct BaseEntry {
    id: u64,
    path: String,
    data: BaseData
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Base {
    name: String,
    tallest: u64,
    bases: Vec<BaseEntry>
}

impl Base {
    pub fn to_json(&self) -> Result<Vec<u8>, std::io::Error> {
	Ok(serde_json::to_vec(self)?)
    }
}

pub enum BasePresence {
    Present(Base),
    Absent
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

fn exists_tuplevec<T: PartialEq, U: PartialEq>(
    a: &Vec<(T, U)>,
    n1: T,
    n2: U
) -> (bool, bool) {
    let mut exists_t: bool = false;
    let mut exists_u: bool = false;
    
    for tuple in a {
	if n1 == tuple.0 {
	    exists_t = true;
	}

	if n2 == tuple.1 {
	    exists_u = true;
	}
    }

    (exists_t, exists_u)
}

fn has_path(bvec: &Vec<BaseEntry>, path: &str) -> bool {
    for entry in bvec {
	if entry.path == path {
	    return true;
	}
    }

    false
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

pub fn produce_base(
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
			ctx: Context::Exists(md_ctx)
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


pub fn base_add(base: &mut Base, path: &str) -> Result<(), std::io::Error> {
    if exists(path)? {
        let mdata: Metadata = metadata(path)?;
	
        if !mdata.is_dir() {
	    if !has_path(&base.bases, path) {
		base.bases.push(
                    BaseEntry {
			id: base.tallest,
			path: path.to_string(),
			data: BaseData::Abstract
                    }
		);

		base.tallest += 1;
	    }
        } else {
            return Err(ErrorKind::InvalidInput.into());
        }
    }

    Ok(())
}

pub fn open_base(name: &str) -> Result<Base, std::io::Error> {
    let basepath: String = format!("./plooglybases/{}", name);

    if !exists("./plooglybases")? {
	create_dir("./plooglybases")?;
    }
    
    if exists(&basepath)? {
	let mut jsondata: String = String::new();
	File::open(&basepath)?.read_to_string(&mut jsondata)?;

	let basedata: Base = serde_json::from_str(&jsondata)?; 

	return Ok(basedata);
    } else {
	return Ok(Base {name: name.to_string(),
			tallest: 0,
			bases: Vec::new()});
    }
}

pub fn base_cut_extension(base: &mut Base, ext: &str) {
    base.bases.retain(|value|
		      value.path.ends_with(&format!(".{}", ext)));
}

pub fn base_cut_extension_inv(base: &mut Base, ext: &str) {
    base.bases.retain(|value|
		      !value.path.ends_with(&format!(".{}", ext)));
}

pub fn base_sort_by_key(base: &mut Base, ext: &str) -> Option<std::io::Error> {
    let mut foundkeys: Vec<(&Vec<u8>, &BaseEntry)> = Vec::new();
    let mut rejectkeys: Vec<&BaseEntry> = Vec::new();
    
    for tbase in &mut base.bases {
	let pinfo: &ProdInfo = match &tbase.data {
	    BaseData::Abstract => return Some(ErrorKind::NotFound.into()),
	    BaseData::Produced(bdata) => bdata
	};

	let ctx: &Vec<(String, Vec<u8>)> = match &pinfo.ctx {
	    Context::Absent => return Some(ErrorKind::NotFound.into()),
	    Context::Exists(tctx) => tctx
	};

	let pairopt: Option<&(String, Vec<u8>)> = ctx.iter().find(|tuple| tuple.0 == ext);

	match pairopt {
	    None => {
		rejectkeys.push(tbase);
	    },
	    
	    Some(tuple) => {
		foundkeys.push((&tuple.1, tbase));
	    }
	}
    }

    foundkeys.sort_by_key(|tuple| String::from_utf8(tuple.0.clone()).unwrap());

    let mut new_basee: Vec<BaseEntry> = Vec::new();

    for found in foundkeys {
	new_basee.push(found.1.clone());
    }

    for excl in rejectkeys {
	new_basee.push(excl.clone());
    }

    base.bases = new_basee;
    
    None
}

pub fn open_base_vec(name: &str) -> Result<Vec<u8>, std::io::Error> {
    let basepath: String = format!("./plooglybases/{}", name);

    if !exists("./plooglybases")? {
	create_dir("./plooglybases")?;
    }
    
    if exists(&basepath)? {
	let mut jsondata: Vec<u8> = Vec::new();
	File::open(&basepath)?.read_to_end(&mut jsondata)?;

	return Ok(jsondata);
    }
    
    Err(ErrorKind::InvalidFilename.into())
}

pub fn save_base(base: &Base) -> Result<(), std::io::Error> {
    let basepath: String = format!("./plooglybases/{}", base.name);
    let tosave: Vec<u8> = base.to_json()?;

    if !exists("./plooglybases")? {
	create_dir("./plooglybases")?;
    }
    
    File::create(&basepath)?.write_all(&tosave)?;

    Ok(())
}

pub fn base_from_json(json: &[u8]) -> Result<Base, std::io::Error> {
    let toret: Base = serde_json::from_slice(&json)?;

    Ok(toret)
}
