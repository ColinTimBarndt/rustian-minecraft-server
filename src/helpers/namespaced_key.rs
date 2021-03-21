use std::borrow::{Borrow, Cow};
use std::hash::{Hash, Hasher};
use std::io::Write;

use super::MINECRAFT_NAMESPACE;

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
    pub fn serialize<W: Write + ?Sized>(&self, buffer: &mut W) {
        use crate::packet::data::write;
        write::var_usize(buffer, self.0.len() + self.1.len() + 1);
        write::string(buffer, self.0.as_ref());
        write::u8(buffer, b':');
        write::string(buffer, self.1.as_ref());
    }
    pub fn serialize_vec<W: Write + ?Sized>(buffer: &mut W, vec: &[NamespacedKey]) {
        use crate::packet::data::write;
        write::var_usize(buffer, vec.len());
        for key in vec {
            key.serialize(buffer);
        }
    }
}

impl Hash for NamespacedKey {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        hasher.write(&self.0.as_bytes());
        hasher.write(&self.1.as_bytes());
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
        let parts = from.split_once(":");
        match parts {
            None => Ok(NamespacedKey::new(MINECRAFT_NAMESPACE, from)),
            Some((ns, key)) => {
                if key.len() == 0 {
                    Err(NamespacedKeyFormatError::MissingKey)
                } else {
                    if key.contains(':') {
                        Err(NamespacedKeyFormatError::InvalidCharacters)
                    } else {
                        Ok(NamespacedKey::new(ns.to_string(), key.to_string()))
                    }
                }
            }
        }
    }
}
