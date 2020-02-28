use std::io::{Error, ErrorKind};
use std::fs::read_dir;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use flate2::bufread::ZlibDecoder;

use crate::id::Id;
use crate::objects::Type;
use crate::datastore::store::LooseStore;

static GIT_DIR: &'static str = ".git";
static OBJECT_DIR: &'static str = "objects";


pub fn from(path: &Path) -> Result<LooseStore, std::io::Error> {
    let mut object_root = std::path::PathBuf::new();
    object_root.push(path);
    object_root.push(GIT_DIR);
    object_root.push(OBJECT_DIR);
    let store = loose_from_path(&object_root);
    store
}

fn loose_from_path(path: &Path) -> Result<LooseStore, std::io::Error> {
    // let object_root: PathBuf = path.clone().to_path_buf();
    let mut object_root = PathBuf::new();
    object_root.push(path);
    let mut filter = [false; 256];
    for e in read_dir(&object_root)? {
        let e = e?;
        let os_filename = e.file_name();
        if os_filename.len() != 2 {
            continue;
        }
        let result = match usize::from_str_radix(&os_filename.to_string_lossy(), 16) {
            Ok(xs) => xs,
            Err(_) => continue
        };
        filter[result] = true;
    }
    let store = LooseStore::new(move |id| {
        let dir = id.to_string();
        let mut pb = object_root.clone();
        pb.push(dir[0..2].to_string());
        match std::fs::File::open(pb.as_path()) {
            Ok(f) => Ok(Some(Box::new(f))),
            Err(_e) => Ok(None)
        }
    }, Some(filter));

    Ok(store)
}

