use crate::packet::{data::write, packet_ids::PLAY_CB_DECLARE_COMMANDS, PacketSerialOut};
use crate::server::universe::commands::parsing::NodeGraph;

#[derive(Debug)]
/// # Declare Commands
/// [Documentation](https://wiki.vg/Protocol#Declare_Commands)
///
/// Lists all of the commands on the server, and how they are parsed.
/// This is a directed graph, with one root node. Each redirect or
/// child node must refer only to nodes that have already been declared.
///
/// For more information on this packet, see the [Command Data](https://wiki.vg/Command_Data) article.
pub struct DeclareCommands<'a> {
  pub command_parsing_graph: &'a NodeGraph,
}

impl PacketSerialOut for DeclareCommands<'_> {
  const ID: u32 = PLAY_CB_DECLARE_COMMANDS;
  fn write(&self, buffer: &mut Vec<u8>) -> Result<(), String> {
    self.command_parsing_graph.serialize_graph(buffer);
    Ok(())
  }
}
