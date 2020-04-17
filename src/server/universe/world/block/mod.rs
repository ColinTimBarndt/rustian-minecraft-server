#![allow(unused)]
use std::collections::HashMap;
extern crate micromath;
extern crate lazy_static;
use std::ops;
use micromath::vector::{F32x3, I32x3};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

pub mod blocks;

pub use blocks::Block;

//type Compound = HashMap<String, nbt::Value>;

/*#[derive(Debug, Clone)]
pub struct BlockPosition {
    x: i32,
    y: i32,
    z: i32
}
impl BlockPosition {
    pub fn get_x(&self) -> i32 {self.x}
    pub fn get_y(&self) -> i32 {self.y}
    pub fn get_z(&self) -> i32 {self.z}
    pub fn set_x(&mut self, x: i32) {self.x = x}
    pub fn set_y(&mut self, y: i32) {self.y = y}
    pub fn set_z(&mut self, z: i32) {self.z = z}
}

impl ops::Add<BlockPosition> for BlockPosition {
    type Output = BlockPosition;
    fn add(self, pos: BlockPosition) -> Self::Output {
        BlockPosition {
            x: self.x + pos.x,
            y: self.y + pos.y,
            z: self.z + pos.z
        }
    }
}
impl ops::Sub<BlockPosition> for BlockPosition {
    type Output = BlockPosition;
    fn sub(self, pos: BlockPosition) -> Self::Output {
        BlockPosition {
            x: self.x - pos.x,
            y: self.y - pos.y,
            z: self.z - pos.z
        }
    }
}

impl From<BlockPosition> for F32x3 {
    fn from(pos: BlockPosition) -> F32x3 {
        F32x3::new(
            pos.x as f32,
            pos.y as f32,
            pos.z as f32
        )
    }
}
impl From<F32x3> for BlockPosition {
    fn from(from: F32x3) -> BlockPosition {
        BlockPosition {
            x: from.x as i32,
            y: from.y as i32,
            z: from.z as i32
        }
    }
}
impl From<I32x3> for BlockPosition {
    fn from(from: I32x3) -> BlockPosition {
        BlockPosition {
            x: from.x,
            y: from.y,
            z: from.z
        }
    }
}*/

/*pub trait Block {
    fn id(&self) -> NamespacedKey;
    fn blast_resistance(&self) -> f32;
    fn break_speed(&self) -> f32;
    fn material(&self) -> BlockMaterial;
    fn map_color(&self) -> MapColor;
    fn new(position: BlockPosition) -> Self;
    fn position<'a>(&'a self) -> &'a BlockPosition;
    fn mut_position<'a>(&'a mut self) -> &'a mut BlockPosition;
    fn has_nbt(&self) -> bool {
        false
    }
    fn set_nbt(&mut self, _tag: Compound) {
        // Empty
    }
    fn get_nbt(&self) -> Option<Compound> {
        None
    }
    fn from_map(position: BlockPosition, state: HashMap<String, String>) -> Result<Box<Self>, Box<&'static str>>;
    fn to_map(&self) -> HashMap<String, String>;
}

pub trait FromState<T: BlockState>: Block {
    fn from_state(position: BlockPosition, state: T) -> Self;
}

pub trait ToState<T: BlockState>: Block {
    fn to_state(self) -> T;
}

macro_rules! register_materials {
    (
        $(
            $name:ident $map_color:literal;
        )*
    ) => (
        pub enum BlockMaterial {
            $(
                $name
            ),*
        }
        impl BlockMaterial {
            pub fn map_color(&self) -> MapColor {
                use BlockMaterial::*;
                match &self {
                    $(
                        $name => FromPrimitive::from_u8($map_color).expect("Invalid map color")
                    ),*
                }
            }
        }
    );
}

register_materials!{
    Air 0;
    StructureVoid 0;
    Portal 0;
    Wool 3;
    Plant 7;
    WaterPlant 12;
    ReplacablePlant 7;
    ReplacableWaterPlant 12;
    Water 12;
    BubbleColumn 12;
    Lava 4;
    PackedIce 8;
    Fire 0;
    Orientable 0;
    Web 3;
    BuildableGlass 0;
    Clay 9;
    Earth 10;
    Grass 1;
    SnowLayer 5;
    Sand 2;
    Sponge 18;
    Wood 13;
    Cloth 3;
    TNT 4;
    Leaves 7;
    Shatterable 0;
    Ice 5;
    Cactus 7;
    Stone 11;
    Ore 6;
    SnowBlock 8;
    Heavy 6;
    Banner 0;
    Piston 11;
    Coral 7;
    Pumpkin 7;
    DragonEgg 7;
    Cake 0;
}*/

macro_rules! register_colors {
    ($($id:literal $color:literal $name:ident;)*) => (
        #[derive(Copy, Clone, Debug, FromPrimitive)]
        pub enum MapColor {
            $($name = $id),*
        }
        impl MapColor {
            fn get_color(&self) -> i32 {
                use MapColor::*;
                match *self {
                    $(
                        $name => $color,
                    )*
                }
            }
        }
    );
}

register_colors!{
    0  0x000000 Black;//b
    1  0x7fb238 GrassGreen;//c
    2  0xf7e9a3 CreamYellow;//d
    3  0xc7c7c7 LighterGray;//e
    4  0xff0000 Red;//f
    5  0xa0a0ff SkyBlue;//g
    6  0xa7a7a7 LightGray;//h
    7  0x007c00 DarkGreen;//i
    8  0xffffff White;//j
    9  0xa4a8b8 SlateGray;//k
    10 0x976d4d LightBrown;//l
    11 0x707070 DarkGray;//m
    12 0x4040ff StrongBlue;//n
    13 0x8f7748 MudBrown;//o
    14 0xfffcf5 CreamWhite;//p
    15 0xd87f33 DarkOrange;//q
    16 0xb24cd8 LightPurple;//r
    17 0x6699d8 LightBlue;//s
    18 0xe5e533 StrongYellow;//t
    19 0x7fcc19 LeafGreen;//u
    20 0xf27fa5 Pink;//v
    21 0x4c4c4c DarkerGray;//w
    22 0x999999 Gray;//x
    23 0x4c7f99 DarkerSkyBlue;//y
    24 0x7f3fb2 DarkPurple;//z
    25 0x334cb2 DarkBlue;//A
    26 0x664c33 DarkerBrown;//B
    27 0x667f33 DarkGrassGreen;//C
    28 0x993333 DarkRed;//D
    29 0x191919 DarkestGray;//E
    30 0xfaee4d StrongCreamYellow;//F
    31 0x5cdbd5 DarkSkyBlue;//G
    32 0x4a80ff LighterStrongBlue;//H
    33 0x00d93a LimeGreen;//I
    34 0x815631 DarkBrown;//J
    35 0x700200 BloodRed;//K
    36 0xd1b1a1 WhiteSkin;//L
    37 0x9f5224 BrownOrange;//M
    38 0x95576c DarkPurpur;//N
    39 0x706c8a DarkSlateGray;//O
    40 0xba8524 Gold;//P
    41 0x677535 OliveGreen;//Q
    42 0xa04d4e DarkPink;//R
    43 0x392923 DarkMudBrown;//S
    44 0x876b62 DarkCreamBrown;//T
    45 0x575c5c DarkCyan;//U
    46 0x7a4958 DarkCreamPurple;//V
    47 0x4c3e5c OldPurple;//W
    48 0x4c3223 Brown;//X
    49 0x4c522a DarkOliveGreen;//Y
    50 0x8e3c2e BrickRed;//Z
    51 0x251610 DarkestBrown;//aa
}
