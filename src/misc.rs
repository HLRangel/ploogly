use std::path::PathBuf;

pub fn path_as_relative(mut path: String) -> PathBuf {
    let mut predir: PathBuf = PathBuf::new();
    predir.push("./out/site");

    if path.starts_with("/") {
        path.remove(0);
    }

    predir.push(path);

    predir
}
