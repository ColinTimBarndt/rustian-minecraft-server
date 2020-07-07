# Actors

## 🧍 MinecraftServer

**Managed by:** Main thread

| Message          | Description                                        |
| ---------------- | -------------------------------------------------- |
| Shutdown         | Shuts down the server                              |
| PlayerDisconnect | Notifies the server that a player has disconnected |
| GetUniverse      | Gets the universe of a player                      |

## 🧍 Player PacketHandler

**Managed by:** [MinecraftServer](#MinecraftServer)

## 🧍 Player PacketReceiver

**Managed by:** [Player PacketHandler](#Player_PacketHandler)

## 🧍 Player PacketSender

**Managed by:** [Player PacketHandler](#Player_PacketHandler)

## 🧍 Universe

**Managed by:** [MinecraftServer](#MinecraftServer)

| Message      | Description                                                          |
| ------------ | -------------------------------------------------------------------- |
| StopActor    | Stops the actor thread, returning its structure to the manager       |
| CreateWorld  | Creates a new BlockWorld in this universe                            |
| GetWorld     | Clones the world handle and sends it back to the requester           |
| GetTags      | Clones the universe tags `Arc`s and sends them back to the requester |
| CreatePlayer | Creates a new EntityPlayer and sends the EID back to the requester   |
| RemovePlayer | Removes a player from the universe                                   |

## 🧍 BlockWorld

**Managed by:** [Universe](#Universe)

| Message       | Description                                                    |
| ------------- | -------------------------------------------------------------- |
| StopActor     | Stops the actor thread, returning its structure to the manager |
| GetBlockAtPos | Gets the requested block and sends it back to the requester    |

## 🧍 Region

**Managed by:** [BlockWorld](#BlockWorld)

| Message        | Description                                                    |
| -------------- | -------------------------------------------------------------- |
| StopActor      | Stops the actor thread, returning its structure to the manager |
| GetBlockAtPos  | Gets the requested block and sends it back to the requester    |
| SendChunk      | Sends a chunk packet using the given `PacketSender`            |
| SendLight      | Sends a chunk light packet using the given `PacketSender`      |
| SendChunkMulti | Sends the marked chunks, see SendChunk                         |
| SendLightMulti | Sends the marked chunk's light, see SendLight                  |
