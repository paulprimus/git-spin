use std::collections::HashMap;
use crate::identity::Identity;

#[derive(Debug)]
pub struct Commit {
    attributes: HashMap<Vec<u8>, Vec<Vec<u8>>>,
    committer: Option<Identity>,
    author: Option<Identity>,
    message: Vec<u8>
}

impl Commit {
    pub fn message(&self) -> &[u8] {
        self.message.as_slice()
    }

}