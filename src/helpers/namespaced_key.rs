use std::hash::{Hash, Hasher};

pub struct NamespacedKey(String, String);

impl NamespacedKey {
    pub fn new(namespace: &str, key: &str) -> Self {
        Self(String::from(namespace), String::from(key))
    }
    pub fn namespace(&self) -> &String {&self.0}
    pub fn key(&self) -> &String {&self.1}
}

impl Hash for NamespacedKey {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        let namespace: Vec<u8> = self.0.bytes().collect();
        hasher.write(&namespace);
        let key: Vec<u8> = self.1.bytes().collect();
        hasher.write(&key);
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
    InvalidCharacters
}

impl std::fmt::Display for NamespacedKeyFormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        use NamespacedKeyFormatError::*;
        write!(f, "{}", match self {
            MissingNamespace => "Missing namespace (namespace:key)",
            MissingKey => "Missing key (namespace:key)",
            InvalidCharacters => "Invalid characters in namespaced key"
        })
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
            2 => Ok(NamespacedKey(parts[0].to_string(), parts[1].to_string())),
            _ => Err(NamespacedKeyFormatError::InvalidCharacters)
        }
    }
}