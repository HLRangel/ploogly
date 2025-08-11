/*use std::collections::HashMap;
use std::fs::exists;

struct ProdInfo {
    hash:   u64,
    data:   Vec<u8>,
    kv:     HashMap<String, Vec<u8>>
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
    bases: Vec<BaseEntry>
}

fn base_to_file(base: &Base, path: &str) {

}

fn base_from_file(path: &str) -> Base {

}

fn base_add(base: &Base, path: &str) {
    if exists(path) {

    }
}*/