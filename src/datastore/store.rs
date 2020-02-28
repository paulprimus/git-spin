use std::io::{BufRead, BufReader, Error, ErrorKind, Write};

use flate2::bufread::ZlibDecoder;

use crate::id::Id;
use crate::objects::{Object, Type};

type Reader = dyn Fn(&Id) -> Result<Option<Box<dyn std::io::Read>>, Error>;

pub struct LooseStore {
    read: Box<Reader>,
    filter: [bool; 256],
}

impl LooseStore {
    pub fn new<C>(func: C, filter: Option<[bool; 256]>) -> Self where C: Fn(&Id) -> Result<Option<Box<dyn std::io::Read>>, Error> + 'static {
        let filter = match filter {
            Some(v) => v,
            None => [true; 256]
        };
        LooseStore {
            read: Box::new(func),
            filter,
        }
    }

    pub fn get<W: Write>(&self, id: &Id, output: &mut W) -> Result<Option<Type>, std::io::Error> {
        let maybe_reader = (self.read)(id)?;
        if maybe_reader.is_none() {
            return Ok(None);
        }

        let mut type_vec = Vec::new();
        let mut size_vec = Vec::new();
        let mut reader = BufReader::new(ZlibDecoder::new(BufReader::new(maybe_reader.unwrap())));

        reader.read_until(0x20, &mut type_vec);
        reader.read_until(0, &mut size_vec);

        let loaded_type = match &type_vec[..] {
            b"commit " => Type::Commit,
            b"blob " => Type::Blob,
            b"tree " => Type::Tree,
            b"tag " => Type::Tag,
            &_ => return Err(ErrorKind::InvalidData.into())
        };
        std::io::copy(&mut reader, output)?;
        Ok(Some(loaded_type))
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use crate::id::Id;
    use crate::objects::Object;
    use crate::datastore::store::LooseStore;

    #[test]
    fn read_commit_works() {
        let store = LooseStore::new(include_bytes!("../../fixtures/loose_commit"), None);
        //let storage_set = StorageSet::new(());

        let mut stream = Vec::new();
        let option = store.get(&Id::default(), &mut stream).expect("it exploded");
        if let Some(xs) = option {
            let mut readable = Cursor::new(stream);
            let object = xs.load(&mut readable).expect("failed to load");

            if let Object::Commit(commit) = object {
                let message = std::str::from_utf8(commit.message()).expect("not utf8");
                assert_eq!(message, "maybe implement loose store\n");
            } else {
                panic!("expected commit");
            }
        } else {
            panic!("explode");
        }
    }
}