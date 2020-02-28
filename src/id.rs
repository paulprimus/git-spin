
#[derive(Default,PartialEq, Eq, PartialOrd, Clone, Copy)]
pub struct Id {
    bytes: [u8; 20]
}

impl AsRef<[u8]> for Id {
    fn as_ref(&self) -> &[u8] {
        &self.bytes
    }
}

impl Id {

    pub fn to_string(&self) -> String {
        self.bytes.iter().map(ToString::to_string).collect()
    }
}



#[test]
fn id_default_works() {
    let hash: String = Id::default().to_string();
    assert_eq!(hash, "00000000000000000000");
}






