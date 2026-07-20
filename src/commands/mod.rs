pub mod if_n_def;
pub mod include;
pub mod redir;
pub mod set;
pub mod template;
pub mod truncate;
pub mod unset;
pub mod var;
pub mod iter_dir;
pub mod add_document;
pub mod produce_base;
pub mod load_base;
pub mod cutbase_extension;
pub mod sortbase_by_key;
pub mod iter_base;
pub mod gen_doc;
pub mod reverse_base_order;
pub mod ltrim;
pub mod rtrim;
pub mod call;
pub mod macros;
pub mod append_data_to_file;
pub mod command_table;

use std::collections::HashMap;

/// Bundles all the common mutable and immutable references
/// that a template command needs.
pub struct CommandContext<'a> {
    pub origin: &'a [u8],
    pub current: usize,
    pub last: usize,
    pub vars: &'a mut HashMap<String, Vec<u8>>,
    pub anon_stack: &'a mut Vec<Vec<u8>>,
    pub result: &'a mut Vec<u8>,
}
