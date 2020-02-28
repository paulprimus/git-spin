use std::path::PathBuf;

// local
mod datastore;
mod id;
mod objects;
mod identity;

use datastore::gitfs;


fn main() {
    println!("Welcome to Git Spin");
    let current_dir: PathBuf = match std::env::current_dir() {
        Ok(v) => v,
        Err(_e) => {
            println!{"Das aktuelle Verzeichnis konnte nicht geöffnet werden"}
            return;
        }
    };
    let store = gitfs::from(&current_dir).expect("Store konnte nicht erstellt werden!");
}

#[test]
fn test1() {
    assert_eq!("test", "test");
}




