#![allow(unused)]

use either::Either;
use std::collections::HashMap;

pub mod attribute;

use attribute::*;

pub trait BlockState where Self: std::marker::Sized {
    fn from_map(from: HashMap<String, String>) -> Result<Self, Box<&'static str>>;
    fn from_vec(from: Vec<(String, String)>) -> Result<Self, Box<&'static str>>;
    fn to_map(&self) -> HashMap<String, String>;
}

macro_rules! block_states {
    (
        $(
            $e_name:ident =>
            $(
                $name:ident $type:ty
            ),+
            ;
        )*
    ) => {
        $(
            pub struct $e_name {
                $(
                    pub $name: $type
                ),*
            }
            impl BlockState for $e_name {
                fn from_map(from: HashMap<String, String>) -> Result<Self, Box<&'static str>> {
                    $(
                        let mut $name: $type = <$type>::default_value();
                    )*
                    for (n, v) in from {
                        match n.as_str() {
                            $(
                                stringify!($name) => {
                                    $name = <$type>::read(v.as_str());
                                },
                            )*
                            _ => ()
                        }
                    }
                    Ok(
                        Self {
                            $(
                                $name
                            ),*
                        }
                    )
                }
                fn from_vec(from: Vec<(String, String)>) -> Result<Self, Box<&'static str>> {
                    $(
                        let mut $name: $type = <$type>::default_value();
                    )*
                    for (n, v) in from {
                        match n.as_str() {
                            $(
                                stringify!($name) => {
                                    $name = <$type>::read(v.as_str());
                                },
                            )*
                            _ => ()
                        }
                    }
                    Ok(
                        Self {
                            $(
                                $name
                            ),*
                        }
                    )
                }
                fn to_map(&self) -> HashMap<String, String> {
                    let mut map = HashMap::new();
                    $(
                        map.insert(stringify!($name).to_string(), self.$name.write());
                    )*
                    map
                }
            }
        )*
    };
}

block_states!{
    Waterlogged => waterlogged bool;
    FacingCardinal => facing CardinalDirection; // Anvil, Carved Pumpkin, Glazed Terracotta, Jack o'Lantern, Loom, Wall Heads, Stonecutter
    FacingCardinalAxis => facing CardinalAxis; // Nether Portal
    FacingCardinalWaterlogged => facing CardinalDirection, waterlogged bool; // Ender Chest, Ladder
    ConnectCardinalWaterlogged => east bool, north bool, south bool, waterlogged bool, west bool; // Fences, Glass Panes, Iron Bars
    FacingAll => facing Direction; // End Rod, Jigsaw Block, Shulker Box
    ConnectAll => down bool, east bool, north bool, south bool, up bool, west bool; // Mushroom Blocks
    TallBlock => half TallBlockHalf; // Large Flowers, Tall Grass, Large Fern, Tall Seagrass
    Bamboo => age Num2, leaves BambooLeavesSize, stage Num2;
    Rotatable => rotation Rotation16; // Banner, Heads
    RotatableWaterlogged => rotation Rotation16, waterlogged bool; // Sign
    WallBanner => facing CardinalDirection;
    Barrel => facing Rotation16, open bool;
    Bed => facing CardinalDirection, occupied bool, part BedPart;
    Cocoa => age Num3, facing CardinalDirection;
    FourStageAgeable => age Num4; // Nether Warts, Beetroots, Frosted Ice, Sweet Berry Bush
    EightStageAgeable => age Num8; // Carrots, Wheat, Potatoes
    SixStageAgeable => age Num6; // Chorus Flower
    SixteenStageAgeable => age Num16; // Cactus, Sugar Cane
    TwentySixAgeable => age Num26; // Kelp
    Bell => attachment BellAttachment, facing CardinalDirection;
    AxisAligned => axis Axis; // Logs, Hay Bale, Pillars
    BrewingStand => has_bottle_0 bool, has_bottle_1 bool, has_bottle_2 bool;
    BubbleColumn => drag bool;
    Button => face Attachment3, facing CardinalDirection, powered bool;
    Campfire => facing CardinalDirection, lit bool, signal_fire bool, waterlogged bool;
    Cake => bites Num7;
    Cauldron => level Num4;
    Chest => facing CardinalDirection, r#type ChestType, waterlogged bool;
    CommandBlock => conditional bool, facing Direction;
    Composter => level Num9;
    DaylightDetector => inverted bool, power Num16;
    Dispenser => facing Direction, triggered bool;
    Door => facing CardinalDirection, half TallBlockHalf, hinge Side, open bool, powered bool;
    EndPortalFrame => eye bool, facing CardinalDirection;
    Farmland => moisture Num8;
    FenceGate => facing CardinalDirection, in_wall bool, open bool, powered bool;
    Fire => age Num16, east bool, north bool, south bool, up bool, west bool;
    Furnace => facing CardinalDirection, lit bool;
    Snowable => snowy bool;
    Grindstone => face Attachment3, facing CardinalDirection;
    Hopper => enabled bool, facing HopperDirection;
    Jukebox => has_record bool;
    Lantern => hanging bool;
    Fluid => level Num16;
    Leaves => distance Num8, persistent bool;
    Lectern => facing CardinalDirection, has_book bool, powered bool;
    Lever => face Attachment3, facing CardinalDirection, powered bool;
    NoteBlock => instrument Instrument, note Num26, powered bool;
    Observer => facing Direction, powered bool;
    Piston => extended bool, facing Direction;
    MovingPiston => facing Direction, r#type PistonType;
    PistonHead => facing Direction, short bool, r#type PistonType;
    Powerable => powered bool;
    WeightedPressurePlate => power Num16;
    Rail => shape RailShape;
    RedstoneRail => powered bool, shape StraightRailShape;
    RedstoneComparator => facing CardinalDirection, mode ComparatorMode, powered bool;
    Dust => east DustConnection, north DustConnection, power Num16, south DustConnection, west DustConnection;
    ConditionalLight => lit bool; // Redstone Lamp, Redstone Ore
    RedstoneRepeater => delay Count4, facing CardinalDirection, locked bool, powered bool;
    Sapling => stage Num2;
    Scaffolding => bottom bool, distance Num8, waterlogged bool;
    SeaPickle => pickles Count4, waterlogged bool;
    Slab => r#type SlabType, waterlogged bool;
    SnowLayer => layers Count8;
    Stair => facing CardinalDirection, half BlockHalf, shape StairShape, waterlogged bool;
    StructureBlock => mode StructureBlockMode;
    TNT => unstable bool;
    Trapdoor => facing CardinalDirection, half BlockHalf, open bool, powered bool, waterlogged bool;
    Tripwire => attached bool, disarmed bool, east bool, north bool, powered bool, south bool, west bool;
    TripwireHook => attached bool, facing CardinalDirection, powered bool;
    TurtleEgg => eggs Count4, hatch Num3;
    Vines => east bool, north bool, south bool, up bool, west bool;
    WallTorch => facing CardinalDirection;
    Wall => east bool, north bool, south bool, up bool, waterlogged bool, west bool;
}

// Melom+Pumpkin Stem
pub struct PlantStem(Either<Num8, CardinalDirection>);
impl BlockState for PlantStem {
    fn from_map(map: HashMap<String, String>) -> Result<Self, Box<&'static str>> {
        if let Some(s) = map.get(&"age".to_string()) {
            return Ok(Self(Either::Left(Num8::read(s.as_str()))));
        }
        if let Some(s) = map.get(&"facing".to_string()) {
            return Ok(Self(Either::Right(CardinalDirection::read(s.as_str()))));
        }
        Err(Box::new("Illegal state"))
    }
    fn from_vec(vec: Vec<(String, String)>) -> Result<Self, Box<&'static str>> {
        for (k, v) in vec {
            if k == "age".to_string() {
                return Ok(Self(Either::Left(Num8::read(v.as_str()))));
            }
            if k == "facing".to_string() {
                return Ok(Self(Either::Right(CardinalDirection::read(v.as_str()))));
            }
        }
        Err(Box::new("Illegal state"))
    }
    fn to_map(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        match &self.0 {
            Either::Left(num) => {
                map.insert("age".to_string(), num.write());
            },
            Either::Right(dir) => {
                map.insert("facing".to_string(), dir.write());
            }
        }
        map
    }
}
impl PlantStem {
    pub fn age(&self) -> u8 {
        if let Either::Left(num) = &self.0 {
            **num
        } else {
            7
        }
    }
}

// Redstone Torch
pub struct ConditionalWallLight(bool, Option<CardinalDirection>);
impl BlockState for ConditionalWallLight {
    fn from_map(map: HashMap<String, String>) -> Result<Self, Box<&'static str>> {
        let lit = bool::read(map.get(&"lit".to_string()).expect("Illegal state").as_str());
        if let Some(s) = map.get(&"facing".to_string()) {
            Ok(Self(lit, Option::Some(CardinalDirection::read(s.as_str()))))
        } else {
            Ok(Self(lit, Option::None))
        }
    }
    fn from_vec(vec: Vec<(String, String)>) -> Result<Self, Box<&'static str>> {
        let mut lit = false;
        let mut facing = Option::None;
        for (k, v) in vec {
            if k == "lit".to_string() {
                lit = bool::read(v.as_str());
                continue;
            }
            if k == "facing".to_string() {
                facing = Option::Some(CardinalDirection::read(v.as_str()));
                continue;
            }
        }
        Ok(Self(lit, facing))
    }
    fn to_map(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("lit".to_string(), self.0.write());
        if let Some(d) = &self.1 {
            map.insert("facing".to_string(), d.write());
        }
        map
    }
}
impl ConditionalWallLight {
    pub fn on_wall(&self) -> bool {
        match self.1 {
            Option::Some(_) => true,
            Option::None => false
        }
    }
}

pub struct Plain;
impl BlockState for Plain {
    fn from_map(_: HashMap<String, String>) -> Result<Self, Box<&'static str>> {
        Ok(Self)
    }
    fn from_vec(_: Vec<(String, String)>) -> Result<Self, Box<&'static str>> {
        Ok(Self)
    }
    fn to_map(&self) -> HashMap<String, String> {
        HashMap::new()
    }
}

impl Fluid {
    fn is_falling(&self) -> bool {
        *self.level>7
    }
    fn set_falling(&mut self, f: bool) {
        let c = self.is_falling();
        if c == f {
            return;
        }
        if c {
            self.level.set(*self.level-8);
        } else {
            self.level.set(*self.level+8);
        }
    }
    fn get_level(&self) -> u8 {
        if self.is_falling() {
            *self.level-8
        } else {
            *self.level
        }
    }
    fn set_level(&mut self, n: u8) {
        assert!(n<8);
        if self.is_falling() {
            self.level.set(n+8);
        } else {
            self.level.set(n);
        }
    }
}
