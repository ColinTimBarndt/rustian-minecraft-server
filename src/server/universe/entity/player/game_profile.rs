use std::borrow::Cow;
use std::collections::HashMap;
use uuid::Uuid;

type CowStr = Cow<'static, str>;

pub static TEXTURES_PROPERTY_NAME: &'static str = "textures";
static OFFLINE_PROFILE_SEED: &'static [u8; 14] = b"OfflinePlayer:";

#[derive(Debug, Clone)]
pub struct GameProfile {
  pub uuid: Uuid,
  pub name: String,
  pub properties: HashMap<CowStr, GameProfileProperty>,
}

#[derive(Debug, Clone, Hash)]
pub struct GameProfileProperty {
  pub name: CowStr,
  pub value: String,
  pub signature: Option<String>,
}

impl GameProfile {
  pub fn new(uuid: Uuid, name: &str, properties: Vec<GameProfileProperty>) -> Self {
    let mut map = HashMap::with_capacity(properties.len());
    for prop in properties {
      map.insert(prop.name.clone(), prop);
    }
    Self {
      uuid,
      name: name.to_owned(),
      properties: map,
    }
  }
  /// See [Protocol: Spawn Player](https://wiki.vg/Protocol#Spawn_Player)
  pub fn new_offline(name: &str) -> Self {
    let mut seed = Vec::with_capacity(OFFLINE_PROFILE_SEED.len() + name.len());
    seed.extend_from_slice(OFFLINE_PROFILE_SEED);
    seed.extend(name.bytes());
    Self {
      name: name.to_string(),
      // The namespace used was guessed because it is undocumented
      uuid: Uuid::new_v3(&Uuid::NAMESPACE_OID, &seed),
      ..Default::default()
    }
  }
}

impl GameProfileProperty {
  pub fn new(name: &str, value: &str, signature: Option<String>) -> Self {
    // Avoid allocating
    let cow_name = {
      if name == TEXTURES_PROPERTY_NAME {
        Cow::Borrowed(TEXTURES_PROPERTY_NAME)
      } else {
        // Unknown property. Need to allocate
        Cow::Owned(String::from(name))
      }
    };
    Self {
      name: cow_name,
      value: value.to_owned(),
      signature,
    }
  }
}

impl Default for GameProfile {
  fn default() -> Self {
    Self {
      uuid: uuid::Builder::nil().build(),
      name: "".to_string(),
      properties: HashMap::new(),
    }
  }
}
