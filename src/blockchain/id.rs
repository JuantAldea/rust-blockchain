#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Id {
    pub id: String,
    id_hash: String,
}

impl Id {
    pub fn new(id: &str) -> Self {
        let mut bytes = vec![];
        bytes.extend(id.as_bytes());
        let id_hash = crypto_hash::hex_digest(crypto_hash::Algorithm::SHA256, &bytes);
        Self {
            id: id.to_string(),
            id_hash,
        }
    }
}

use std::fmt;
impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.id_hash[..10])
    }
}
