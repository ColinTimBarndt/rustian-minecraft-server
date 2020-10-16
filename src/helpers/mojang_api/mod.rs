static SESSION_SERVER_BASE_URL: &'static str = "https://sessionserver.mojang.com/session/minecraft";

mod hexdigest;

use crate::server::universe::entity::player::game_profile::{GameProfile, GameProfileProperty};
use hexdigest::calc_hash;
use openssl::pkey::Private;
use openssl::rsa::Rsa;
use reqwest::Url;
use serde::Deserialize;
use std::net::SocketAddr;
use std::sync::Arc;

fn create_joined_url(username: &str, hash: &str, ip: &str) -> Url {
  Url::parse_with_params(
    &format!("{base}/hasJoined", base = SESSION_SERVER_BASE_URL),
    &[("username", username), ("serverId", hash), ("ip", ip)],
  )
  .unwrap()
}

#[derive(Debug)]
pub enum Error {
  ServiceUnavailable,
  MalformedResponse,
}

/// Gets the player's profile if the player has valid
/// Minecraft account credentials
pub async fn has_joined(
  server_id: &str,
  shared_secret: &[u8],
  key: &Arc<Rsa<Private>>,
  username: &str,
  user_ip: &SocketAddr,
) -> Result<Option<GameProfile>, Error> {
  let ip_str = user_ip.ip().to_string();
  let hash = calc_hash(&[
    server_id.as_bytes(),
    shared_secret,
    &rsa_der::public_key_to_der(&mut key.n().to_vec(), &mut key.e().to_vec()),
  ]);

  let client = reqwest::Client::new();
  let response_raw = client
    .get(create_joined_url(username, &hash, &ip_str))
    .send()
    .await
    .map_err(|_| Error::ServiceUnavailable)?;

  if response_raw.status() == 204 {
    return Ok(None);
  }

  let response = response_raw
    .json::<HasJoinedResponse>()
    .await
    .map_err(|_| Error::MalformedResponse)?;

  let uuid = {
    use std::str::FromStr;
    uuid::Uuid::from_str(&response.id).map_err(|_| Error::MalformedResponse)
  }?;

  Ok(Some(GameProfile::new(
    uuid,
    &response.name,
    response
      .properties
      .into_iter()
      .map(|prop| GameProfileProperty::new(&prop.name, &prop.value, prop.signature))
      .collect(),
  )))
}

#[derive(Deserialize)]
struct HasJoinedResponse {
  pub id: String,
  pub name: String,
  pub properties: Vec<HasJoinedResponseProperty>,
}

#[derive(Deserialize)]
struct HasJoinedResponseProperty {
  pub name: String,
  pub value: String,
  pub signature: Option<String>,
}
