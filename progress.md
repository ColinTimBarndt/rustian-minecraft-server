# Progress log

## Implement [login sequence](https://wiki.vg/Protocol_FAQ#What.27s_the_normal_login_sequence_for_a_client.3F)

| ❌  | Step                                                                                                                            |
| :-: | ------------------------------------------------------------------------------------------------------------------------------- |
| ✅  | Client connects to server                                                                                                       |
| ✅  | C→S: Handshake State=2                                                                                                          |
| ✅  | C→S: Login Start                                                                                                                |
| ✅  | S→C: Encryption Request                                                                                                         |
| ✅  | Client auth                                                                                                                     |
| ✅  | C→S: Encryption Response                                                                                                        |
| ✅  | Server auth, both enable encryption                                                                                             |
| ❌  | S→C: Set Compression (Optional, enables compression)                                                                            |
| ✅  | S→C: Login Success                                                                                                              |
| ✅  | S→C: Join Game                                                                                                                  |
| ✅  | S→C: Plugin Message: minecraft:brand with the server's brand (Optional)                                                         |
| ✅  | S→C: Server Difficulty (Optional)                                                                                               |
| ✅  | S→C: Player Abilities (Optional)                                                                                                |
| ✅  | C→S: Plugin Message: minecraft:brand with the client's brand (Optional)                                                         |
| ✅  | C→S: Client Settings                                                                                                            |
| ✅  | S→C: Held Item Change                                                                                                           |
| ➕  | C→S: Held Item Change (serverbound, Optional?) _(The wiki does not mention this)_                                               |
| ✅  | S→C: Declare Recipes                                                                                                            |
| ✅  | S→C: Tags                                                                                                                       |
| ✅  | S→C: Entity Status _(The wiki does not specify which status, assuming op permission level)_                                     |
| ✅  | S→C: Declare Commands                                                                                                           |
| ✅  | S→C: Unlock Recipes                                                                                                             |
| 🟡  | S→C: ~~Player Position And Look~~ _(This is not true, the first position packet will make the player leave the loading screen)_ |
| ❌  | S→C: Player Info (Add Player action)                                                                                            |
| ❌  | S→C: Player Info (Update latency action)                                                                                        |
| ✅  | S→C: Update View Position                                                                                                       |
| ✅  | S→C: Update Light (One sent for each chunk in a square centered on the player's position)                                       |
| ✅  | S→C: Chunk Data (One sent for each chunk in a square centered on the player's position)                                         |
| ✅  | S→C: World Border (Once the world is finished loading)                                                                          |
| ✅  | S→C: Spawn Position (“home” spawn, not where the client will spawn on login)                                                    |
| ✅  | S→C: Player Position And Look (Required, tells the client they're ready to spawn)                                               |
| ✅  | C→S: Teleport Confirm                                                                                                           |
| ✅  | C→S: Player Position And Look (to confirm the spawn position)                                                                   |
| ❌  | C→S: Client Status (sent either before or while receiving chunks, further testing needed, server handles correctly if not sent) |
| ❌  | S→C: inventory, entities, etc                                                                                                   |
