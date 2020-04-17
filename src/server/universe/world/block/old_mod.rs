use std::collections::HashMap;
use num_traits::FromPrimitive;
use crate::server::universe::world::{
    NamespacedKey,
    Block,
    block::{
        FromState,
        ToState,
        BlockPosition,
        BlockMaterial,
        MapColor
    },
    block_state::{
        self,
        BlockState
    }
};

macro_rules! register_blocks {
    (
        $(
            block{
                $B:ident
                $id:literal
                $state:ident
                $blast_resistance:literal
                $break_speed:literal
                $material:ident
                $map_color:literal
            }
        )+
    ) => (
        pub enum BlockEnum {
            $(
                $B($B)
            ),*
        }

        $(
            block!{
                $B $id $state;
                blast_resistance $blast_resistance,
                break_speed $break_speed
                material $material
                map_color $map_color
            }
        )*
    );
}

macro_rules! block {
    (
        $B:ident
        $id:literal
        $state:ident
        $blast_resistance:literal
        $break_speed:literal
        $material:ident
        $map_color:literal
    ) => (
        block!{
            $B $id $state;
            blast_resistance $blast_resistance, // Wiki / 5
            break_speed $break_speed
            material $material
            map_color $map_color
        }
    );
    (
        $B:ident $id:literal $state:ident;
        blast_resistance $blast_resistance:literal,
        break_speed $break_speed:literal
        material $material:ident
        map_color $map_color:literal
    ) => {
        pub struct $B {
            position: BlockPosition,
            pub state: block_state::$state
        }
        impl Block for $B {
            fn id(&self) -> NamespacedKey {
                NamespacedKey("minecraft".to_string(), $id.to_string())
            }
            fn blast_resistance(&self) -> f32 {
                $blast_resistance
            }
            fn break_speed(&self) -> f32 {
                $break_speed
            }
            fn material(&self) -> BlockMaterial {
                BlockMaterial::$material
            }
            fn map_color(&self) -> MapColor {
                FromPrimitive::from_u8($map_color).expect("Invalid map color")
            }
            fn new(_: BlockPosition) -> Self { unimplemented!() }
            fn position<'a>(&'a self) -> &'a BlockPosition {
                &self.position
            }
            fn mut_position<'a>(&'a mut self) -> &'a mut BlockPosition {
                &mut self.position
            }
            fn from_map(position: BlockPosition, state: HashMap<String, String>) -> Result<Box<Self>, Box<&'static str>> {
                Ok(Box::new(
                    Self {
                        position,
                        state: block_state::$state::from_map(state)?
                    }
                ))
            }
            fn to_map(&self) -> HashMap<String, String> {
                self.state.to_map()
            }
        }
        impl FromState<block_state::$state> for $B {
            fn from_state(position: BlockPosition, state: block_state::$state) -> Self {
                Self {
                    position,
                    state
                }
            }
        }
        impl ToState<block_state::$state> for $B {
            fn to_state(self) -> block_state::$state {
                self.state
            }
        }
    };
}

register_blocks!{
    block{Stone "stone" Plain 6.0 1.5 Stone 11}

    block{Granite "granite" Plain 6.0 1.5 Stone 10}
    block{PolishedGranite "polished_granite" Plain 6.0 1.5 Stone 10}

    block{Diorite "diorite" Plain 6.0 1.5 Stone 14}
    block{PolishedDiorite "polished_diorite" Plain 6.0 1.5 Stone 14}

    block{Andesite "andesite" Plain 6.0 1.5 Stone 11}
    block{PolishedAndesite "polished_andesite" Plain 6.0 1.5 Stone 11}

    block{GrassBlock "grass_block" Snowable 0.6 0.6 Grass 1}
    block{Dirt "dirt" Plain 0.5 0.5 Earth 10}
    block{CoarseDirt "coarse_dirt" Plain 0.5 0.5 Earth 10}
    block{Podzol "podzol" Snowable 0.5 0.5 Earth 34}

    block{Cobblestone "cobblestone" Plain 6.0 2.0 Stone 11}
}

