


mod commit;


#[derive(Copy, Clone, Debug)]
pub enum Type {
    Commit,
    Tree,
    Blob,
    Tag
}

pub enum Object {
    Commit(commit::Commit),
    // Tree(tree::Tree),
    // Blob(blob::Blob),
    // Tag(tag::Tag)
}