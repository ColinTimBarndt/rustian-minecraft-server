use crate::helpers::{NamespacedKey, MINECRAFT_NAMESPACE};
use crate::packet::data::write;
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

mod number_bounds;
pub use number_bounds::*;

#[derive(Clone, Debug)]
pub struct NodeGraph {
  nodes: HashMap<Uuid, Node>,
  root: Option<Uuid>,
}

#[derive(Clone, Debug)]
pub struct Node {
  pub node_type: NodeType,
  /// Whether the command is executeble if this
  /// is the last node in the command
  pub executable: bool,
  /// Id of this node. The id has to be unique in the graph
  id: Uuid,
  // Used to connect this node as a child to a parent
  // The parent has to list it in the parents Vec
  children: HashSet<Uuid>,
  parents: HashSet<Uuid>,
  // Used to redirect from this node to another one
  // The node that is redirected to has to list it
  // in its `redirected_from` property
  redirects_to: Option<Uuid>,
  redirected_from: HashSet<Uuid>,
}

#[derive(Clone, Debug, Hash)]
pub enum NodeType {
  /// Graph root node
  Root,
  /// Literal (name)
  Literal(String),
  /// Argument
  Argument(ArgumentNodeType),
}

impl NodeGraph {
  pub fn new() -> Self {
    Self {
      nodes: HashMap::new(),
      root: None,
    }
  }
  pub fn with_capacity(capacity: usize) -> Self {
    Self {
      nodes: HashMap::with_capacity(capacity),
      root: None,
    }
  }
  /// Creates a new node and adds it to the graph
  pub fn create_node(&mut self, node_type: NodeType) -> Uuid {
    let uuid = self.create_uuid();
    let node = Node {
      id: uuid,
      node_type: node_type,
      ..Node::new()
    };
    self.nodes.insert(uuid, node);
    uuid.clone()
  }
  /// Creates a new node and calls the mutator before it is
  /// added to the graph
  pub fn create_node_mut(
    &mut self,
    node_type: NodeType,
    mutator: impl Fn(&mut Node) -> (),
  ) -> Uuid {
    let uuid = self.create_uuid();
    let mut node = Node {
      id: uuid,
      node_type: node_type,
      ..Node::new()
    };
    mutator(&mut node);
    self.nodes.insert(uuid, node);
    uuid.clone()
  }
  fn create_uuid(&self) -> Uuid {
    let mut uuid = Uuid::new_v4();
    while self.nodes.contains_key(&uuid) {
      uuid = Uuid::new_v4();
    }
    uuid
  }
  pub fn get_node(&self, uuid: &Uuid) -> Option<&Node> {
    self.nodes.get(uuid)
  }
  pub fn get_node_mut(&mut self, uuid: &Uuid) -> Option<&mut Node> {
    self.nodes.get_mut(uuid)
  }
  pub fn has_node(&self, uuid: &Uuid) -> bool {
    self.nodes.contains_key(uuid)
  }
  /// This function sets one node as the parent of another node.
  /// If the child node already has a parent, then it is moved into
  /// the new relation, removing the child from its previous parent.
  pub fn set_child(&mut self, parent: &Uuid, child: &Uuid) {
    debug_assert!(self.has_node(parent), "Parent is not in this graph");
    debug_assert!(self.has_node(child), "Child is not in this graph");
    let parent_node = self.get_node_mut(parent).unwrap();
    if !parent_node.children.insert(*child) {
      // The parent->child relation already exists
      return;
    }
    drop(parent_node);
    let child_node = self.get_node_mut(child).unwrap();
    let previous_parents = child_node.parents.clone();
    let has_parent = child_node.parents.insert(*parent);
    debug_assert!(
      has_parent,
      "Child node already has parent, but not in reverse!"
    );
    drop(child_node);
    for previous_parent in previous_parents {
      debug_assert!(
        self.nodes.contains_key(&previous_parent),
        "Previous parent is not in this graph"
      );
      let previous_parent_node = self.get_node_mut(&previous_parent).unwrap();
      let had_child = previous_parent_node.children.remove(child);
      debug_assert!(had_child, "Previous parent did not have the child");
    }
  }
  pub fn remove_child(&mut self, parent: &Uuid, child: &Uuid) -> bool {
    debug_assert!(self.has_node(parent), "Root node is not in this graph");
    debug_assert!(self.has_node(child), "Root node is not in this graph");
    let parent_n = self.get_node_mut(parent).unwrap();
    if !parent_n.children.remove(child) {
      return false;
    }
    drop(parent_n);
    let child_n = self.get_node_mut(child).unwrap();
    let parent_removed = child_n.parents.remove(parent);
    debug_assert!(
      parent_removed,
      "Child did not have the parent, but in reverse!"
    );
    drop(child_n);
    true
  }
  /// Sets the root node of this graph. This will set the node
  /// type to Root and safely remove all parents from this node
  pub fn set_root(&mut self, root: &Uuid) {
    debug_assert!(self.has_node(root), "Root node is not in this graph");
    let root_n = self.get_node_mut(root).unwrap();
    root_n.node_type = NodeType::Root;
    let parents = root_n.parents.clone();
    root_n.parents.clear();
    drop(root_n);
    // Mutable reference no longer needed
    //let root_n = &*root_n;
    for parent in &parents {
      let parent_n = self.get_node_mut(parent).unwrap();
      let had_child = parent_n.children.remove(root);
      debug_assert!(had_child, "Previous parent did not have root as a child");
    }
    self.root = Some(*root);
  }
  /// This function sets the redirect of the node if it not already
  /// exists.
  pub fn set_redirect(&mut self, node: &Uuid, redirect: &Uuid) {
    debug_assert!(self.has_node(node), "Node is not in this graph");
    debug_assert!(self.has_node(redirect), "Redirect is not in this graph");
    let node_n = self.get_node_mut(node).unwrap();
    let node_redirect = &mut node_n.redirects_to;
    let previous_redirect = *node_redirect;
    *node_redirect = Some(*redirect);
    if previous_redirect == *node_redirect {
      // Already redirecting to this node.
      return;
    }
    drop(node_n);
    if let Some(previous_redirect) = previous_redirect {
      debug_assert!(
        self.has_node(&previous_redirect),
        "Previously redirected is not in this graph"
      );
      let previous_redirect_node = self.get_node_mut(&previous_redirect).unwrap();
      let had_redirection = previous_redirect_node.redirected_from.remove(node);
      debug_assert!(
        had_redirection,
        "Previously redirected node did not have a reference to the redirecting node"
      );
    }
    let redirect_node = self.get_node_mut(redirect).unwrap();
    let redirect_inserted = redirect_node.redirected_from.insert(*node);
    debug_assert!(
      redirect_inserted,
      "Redirection already existed in the redirected node"
    );
    drop(redirect_node);
  }
  /// This functions removes the redirect of the node. The node
  /// has to be an Argument Node, otherwise this unction panics.
  pub fn remove_redirect(&mut self, node: &Uuid) {
    debug_assert!(self.has_node(node), "Node is not in this graph");
    let node_n = self.get_node_mut(node).unwrap();
    let node_redirect = &mut node_n.redirects_to;
    let previous_redirect = *node_redirect;
    *node_redirect = None;
    drop(node_n);
    if let Some(previous_redirect) = previous_redirect {
      debug_assert!(
        self.has_node(&previous_redirect),
        "Previously redirected is not in this graph"
      );
      let previous_redirect_node = self.get_node_mut(&previous_redirect).unwrap();
      let had_redirection = previous_redirect_node.redirected_from.remove(node);
      debug_assert!(
        had_redirection,
        "Previously redirected node did not have a reference to the redirecting node"
      );
    }
  }
  /// Safely removes a node from the graph. This means that all
  /// other references to this node in the graph are removed.
  pub fn remove_node(&mut self, node: &Uuid) {
    match self.nodes.remove_entry(node) {
      Some((_uuid, node_n)) => {
        // Remove references in parents
        for parent in &node_n.parents {
          debug_assert!(self.has_node(parent), "Parent is not in this graph");
          let parent_n = self.get_node_mut(parent).unwrap();
          let had_child = parent_n.children.remove(node);
          debug_assert!(had_child, "Parent did not have this node as a child");
        }
        // Remove references in redirectors
        for redirector in &node_n.redirected_from {
          debug_assert!(self.has_node(redirector), "Parent is not in this graph");
          let redirector_n = self.get_node_mut(redirector).unwrap();
          let node_redirect = &mut redirector_n.redirects_to;
          if let Some(redirected) = &node_redirect {
            debug_assert_eq!(
              redirected, node,
              "Redirector does not redirect to this node"
            );
          } else {
            panic!("Reidrector does not redirect")
          }
          *node_redirect = None;
        }
        if self.root.as_ref() == Some(node) {
          self.root = None;
        }
      }
      None => (),
    }
  }
  pub fn serialize_graph(&self, buffer: &mut Vec<u8>) {
    assert!(self.root.is_some(), "This graph does not have a root node!");
    let mut sorted: Vec<Uuid> = Vec::with_capacity(self.nodes.len());

    // Traverse the graph to sort the nodes in an order
    // that node childs are declared before the parent.
    {
      let root_id: &Uuid = self.root.as_ref().unwrap();
      sorted.push(*root_id);
      traverse(self, &mut sorted, root_id);

      fn traverse(graph: &NodeGraph, sorted: &mut Vec<Uuid>, node: &Uuid) {
        let node_n: &Node = graph.get_node(node).unwrap();
        for child in &node_n.children {
          traverse(graph, sorted, child);
        }
        sorted.push(*node);
      }
    }

    let mut visited: HashSet<&Uuid> = HashSet::with_capacity(sorted.len());
    let sorted: Vec<&Node> = sorted
      .iter()
      .rev()
      .filter(|uuid| visited.insert(uuid))
      .map(|uuid| self.get_node(uuid).unwrap())
      .collect();
    drop(visited);

    let mappings: HashMap<Uuid, usize> = sorted
      .iter()
      .enumerate()
      .map(|(idx, node)| (node.id, idx))
      .collect();
    // Estimate the amount of data in bytes and allocate it
    // so that the buffer has to allocate less ofter while writing
    // TODO: This could be made more precise
    buffer.reserve(sorted.len() * 26);
    write::var_usize(buffer, sorted.len());
    for node in sorted {
      let flags: u8 =
        // Mask: 0b00000011
        node.node_type.get_type_flag() |
        // Mask: 0b00000100
        ((node.executable as u8) << 2) |
        // Mask: 0b00001000
        ((node.redirects_to.is_some() as u8) << 3) |
        // Mask: 0b00010000 (if type is Argument)
        if let NodeType::Argument(arg) = &node.node_type {
          (arg.suggestion_type.is_some() as u8) << 4
        } else {0}
        ;
      write::u8(buffer, flags);
      write::var_usize(buffer, node.children.len());
      for child_id in &node.children {
        write::var_usize(buffer, *mappings.get(child_id).unwrap());
      }
      if let Some(id) = &node.redirects_to {
        write::var_usize(buffer, *mappings.get(id).unwrap());
      }
      match &node.node_type {
        NodeType::Literal(name) => write::string(buffer, name),
        NodeType::Argument(arg) => {
          write::string(buffer, &arg.name);
          (&arg.parser).serialize_parser(buffer);
          if let Some(suggestion_type) = &arg.suggestion_type {
            write::string(buffer, suggestion_type.get_id().to_string());
          }
        }
        _ => (),
      }
    }
    write::var_usize(buffer, *mappings.get(&(&self.root).unwrap()).unwrap());
  }
}

impl Node {
  // Can't implement Default because a random Uuid is
  // generated and not the same one => this function has side-effects
  fn new() -> Self {
    Self {
      id: Uuid::new_v4(),
      node_type: NodeType::Root,
      executable: false,
      children: HashSet::new(),
      parents: HashSet::new(),
      redirects_to: None,
      redirected_from: HashSet::new(),
    }
  }
  pub fn get_id(&self) -> &Uuid {
    &self.id
  }
  /// Returns whether the node is needed in the graph.
  /// Returns `false` if the node is not a Root node
  /// and does not have any parents redirections.
  pub fn is_required(&self) -> bool {
    if let NodeType::Root = self.node_type {
      true
    } else {
      self.parents.len() > 0 || self.redirected_from.len() > 0
    }
  }
}

impl NodeType {
  pub fn get_type_flag(&self) -> u8 {
    match self {
      Self::Root => 0b00,
      Self::Literal(_) => 0b01,
      Self::Argument(_) => 0b10,
    }
  }
  pub fn same_type(&self, other: &Self) -> bool {
    self.get_type_flag() == other.get_type_flag()
  }
}

#[derive(Clone, Debug, Hash)]
pub struct ArgumentNodeType {
  pub name: String,
  pub parser: ArgumentParser,
  pub suggestion_type: Option<SuggestionType>,
}

#[derive(Clone, Copy, Debug, Hash)]
pub enum ArgumentParser {
  Bool,
  Double(NumberBounds<f64>),
  Float(NumberBounds<f32>),
  Integer(NumberBounds<i32>),
  String(StringType),
  /// @-selector
  Entity(EntitySelectorRestriction),
  /// Player name or player selector (Multiple players possible)
  GameProfile,
  /// Block position, either absolute (`x y z`) or relative (`~x ~y ~z`)
  BlockPosition,
  // TODO: Declare more Parsers
}

impl ArgumentParser {
  pub fn serialize_parser(&self, buffer: &mut Vec<u8>) {
    match self {
      Self::Double(range) => {
        buffer.append(&mut b"brigadier:double".to_vec());
        range.serialize_bounds(buffer);
      }
      Self::Float(range) => {
        buffer.append(&mut b"brigadier:float".to_vec());
        range.serialize_bounds(buffer);
      }
      Self::Integer(range) => {
        buffer.append(&mut b"brigadier:integer".to_vec());
        range.serialize_bounds(buffer);
      }
      Self::String(t) => {
        buffer.append(&mut b"brigadier:string".to_vec());
        write::var_u8(buffer, *t as u8);
      }
      Self::Entity(restrictions) => {
        buffer.append(&mut b"minecraft:entity".to_vec());
        write::u8(buffer, *restrictions as u8);
      }
      other => {
        buffer.append(&mut match other {
          Self::Bool => b"minecraft:bool".to_vec(),
          Self::GameProfile => b"minecraft:game_profile".to_vec(),
          Self::BlockPosition => b"minecraft:block_pos".to_vec(),
          _ => panic!("[commands/parsing/mod.rs] This state shouldn't be reachable"),
        });
      }
    }
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum StringType {
  /// One `word` (separated by a space)
  Word = 0,
  /// `"Quoted text"`
  Quoted = 1,
  /// Reads `text until the end of the command.`
  Greedy = 2,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum EntitySelectorRestriction {
  Unrestricted = 0b00,
  Single = 0b01,
  Players = 0b10,
  SinglePlayer = 0b11,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
/// [Documentation](https://wiki.vg/Command_Data#Suggestions_Types)
pub enum SuggestionType {
  /// Sends the [Tab-Complete](https://wiki.vg/Protocol#Tab-Complete_.28serverbound.29)
  /// packet to the server to request tab completions.
  AskServer,
  /// Suggests all the available recipes.
  AllRecipes,
  /// Suggests all the available sounds.
  AvailableSounds,
  /// Suggests all the summonable entities.
  SummonableEntities,
}

impl SuggestionType {
  pub fn get_id(&self) -> NamespacedKey {
    match self {
      Self::AskServer => NamespacedKey::new(MINECRAFT_NAMESPACE, "ask_server"),
      Self::AllRecipes => NamespacedKey::new(MINECRAFT_NAMESPACE, "all_recipes"),
      Self::AvailableSounds => NamespacedKey::new(MINECRAFT_NAMESPACE, "available_sounds"),
      Self::SummonableEntities => NamespacedKey::new(MINECRAFT_NAMESPACE, "summonable_entities"),
    }
  }
}
