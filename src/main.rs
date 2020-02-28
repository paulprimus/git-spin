use std::path::PathBuf;

// local
mod datastore;
mod id;
mod objects;
mod identity;

use datastore::gitfs;


fn main() {
    println!("Welcome to Git Spin");
    let current_dir: PathBuf = std::env::current_dir().unwrap();
    gitfs::from(&current_dir).expect("Store konnte nicht erstellt werden!");
    //println!("{}", store);
}

#[test]
fn test1() {
    assert_eq!("test", "test");
}




