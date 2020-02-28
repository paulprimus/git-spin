use std::fmt::Error;
use std::fs::read_dir;
use std::io::prelude::*;
use std::io::{ BufReader};
use std::path::{Path, PathBuf};

use flate2::bufread::ZlibDecoder;

use crate::id::Id;
use crate::objects::Type;

static GIT_DIR: &'static str = ".git";
static OBJECT_DIR: &'static str = "objects";

type Reader = dyn Fn(&Id) -> Result<Option<Box<dyn Read>>, Error> ;

pub struct Store {
    read: Box<Reader>,
    filter: [bool; 256],
}

impl Store {
    fn new<C>(func: C, filter: Option<[bool; 256]>) -> Self where C: Fn(&Id) -> Result<Option<Box<dyn Read>>, Error> + 'static {
        let filter = match filter {
            Some(v) => v,
            None => [true; 256]
        };
        Store {
            read: Box::new(func),
            filter,
        }
    }

    fn get<W: Write>(&self, output: &mut W, id: &Id) -> Result<Option<Type>, Error> {
        let maybe_reader = (self.read)(id)?;
        if maybe_reader.is_none() {
            return Ok(None);
        }

        let mut type_vec = Vec::new();
        let mut reader = BufReader::new(ZlibDecoder::new(maybe_reader.unwrap()));
        reader.read_until(0x20, &mut type_vec);

        let loaded_type = match &type_vec[..] {
            b"commit " => Type::Commit,
            b"blob " => Type::Blob,
            b"tree " => Type::Tree,
            b"tag " => Type::Tag,
            //&_ => return Err()
        };
    }
}


pub fn from(path: &Path) -> Result<Store, Error> {
    let mut object_root = std::path::PathBuf::new();
    object_root.push(path);
    object_root.push(GIT_DIR);
    object_root.push(OBJECT_DIR);
    let store = loose_from_path(&object_root);
    store
}

fn loose_from_path(path: &Path) -> Result<Store, Error> {
    // let object_root: PathBuf = path.clone().to_path_buf();
    let mut object_root = PathBuf::new();
    object_root.push(path);
    let mut filter = [false; 256];
    for e in read_dir(&object_root).unwrap() {
        let os_filename = e.unwrap().file_name();
        if os_filename.len() != 2 {
            continue;
        }
        let result = match usize::from_str_radix(&os_filename.to_string_lossy(), 16) {
            Ok(xs) => xs,
            Err(_) => continue
        };
        filter[result] = true;
    }
    let store = Store::new(move |id| {
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

