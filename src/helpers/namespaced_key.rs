use std::borrow::{Borrow, Cow};
use std::hash::{Hash, Hasher};

#[derive(Debug, Eq, PartialEq)]
pub struct NamespacedKey(Cow<'static, str>, Cow<'static, str>);

impl NamespacedKey {
    pub fn new<A: Into<Cow<'static, str>>, B: Into<Cow<'static, str>>>(
        namespace: A,
        key: B,
    ) -> Self {
        Self(namespace.into(), key.into())
    }
    pub fn namespace(&self) -> &Cow<'static, str> {
        self.0.borrow()
    }
    pub fn key(&self) -> &Cow<'static, str> {
        self.1.borrow()
    }
    pub fn serialize_vec(buffer: &mut Vec<u8>, vec: &Vec<NamespacedKey>) {
        use crate::packet::data::write;
        write::var_usize(buffer, vec.len());
        for key in vec {
            write::string(buffer, key.to_string());
        }
    }
}

impl Hash for NamespacedKey {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        let namespace: Vec<u8> = self.0.bytes().collect();
        hasher.write(&namespace);
        let key: Vec<u8> = self.1.bytes().collect();
        hasher.write(&key);
    }
}

impl Clone for NamespacedKey {
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1.clone())
    }
}

impl std::fmt::Display for NamespacedKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}:{}", self.0, self.1)
    }
}

#[derive(Debug, Clone)]
pub enum NamespacedKeyFormatError {
    MissingNamespace,
    MissingKey,
    InvalidCharacters,
}

impl std::fmt::Display for NamespacedKeyFormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        use NamespacedKeyFormatError::*;
        write!(
            f,
            "{}",
            match self {
                MissingNamespace => "Missing namespace (namespace:key)",
                MissingKey => "Missing key (namespace:key)",
                InvalidCharacters => "Invalid characters in namespaced key",
            }
        )
    }
}
impl std::error::Error for NamespacedKeyFormatError {}

impl std::convert::TryFrom<String> for NamespacedKey {
    type Error = NamespacedKeyFormatError;
    fn try_from(from: String) -> Result<Self, Self::Error> {
        let parts = from.split(":").collect::<Vec<&str>>();
        match parts.len() {
            0 => Err(NamespacedKeyFormatError::MissingNamespace),
            1 => Err(NamespacedKeyFormatError::MissingKey),
            2 => Ok(NamespacedKey::new(
                parts[0].to_string(),
                parts[1].to_string(),
            )),
            _ => Err(NamespacedKeyFormatError::InvalidCharacters),
        }
    }
}
