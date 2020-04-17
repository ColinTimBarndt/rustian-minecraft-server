pub trait BlockStateAttribute {
    fn read(from: &str) -> Self;
    fn write(&self) -> String;
    fn default_value() -> Self;
}

macro_rules! impl_block_state_value {
    (
        $type:ty = $def:expr;
        $(
            $vpat:pat => $str:literal, $vstr:pat => $val:expr
        ),*
    ) => {
        impl BlockStateAttribute for $type {
            fn read(from: &str) -> $type {
                match from {
                    "test" => $def,
                    $(
                        $vstr => $val
                    ),*,
                    _ => $def
                }
            }
            fn write(&self) -> String {
                match self {
                    $(
                        $vpat => $str.to_string()
                    ),*
                }
            }
            fn default_value() -> Self {
                $def
            }
        }
    };
    (
        $type:ty = $def:expr, nexh;
        $(
            $vpat:pat => $str:literal, $vstr:pat => $val:expr
        ),*
    ) => {
        impl BlockStateAttribute for $type {
            fn read(from: &str) -> $type {
                match from {
                    $(
                        $vstr => $val
                    ),*,
                    _ => $def
                }
            }
            fn write(&self) -> String {
                match self {
                    $(
                        $vpat => $str.to_string()
                    ),*,
                    _ => panic!("Illegal state")
                }
            }
            fn default_value() -> Self {
                $def
            }
        }
    };
    (
        $type:ty = $def:literal, $inner:ty => $(min $min:literal)? max $max:literal;
    ) => {
        impl BlockStateAttribute for $type {
            fn read(from: &str) -> Self {
                use std::str::FromStr;
                match <$inner>::from_str(from) {
                    Ok(n) => {
                        if $(n<$min || )? n>$max {
                            Self($def)
                        } else {
                            Self(n)
                        }
                    },
                    Err(_e) => {
                        Self($def)
                    }
                }
            }
            fn write<'a>(&'a self) -> String {
                let n: $inner = self.0;
                if $(n<$min || )? n>$max {
                    panic!("Illegal state");
                } else {
                    n.to_string()
                }
            }
            fn default_value() -> Self {
                Self($def)
            }
        }
    };
}

impl_block_state_value!{
    bool = false;
    false => "false", "false" => false,
    true  => "true",  "true"  => true
}

pub enum CardinalDirection {North, East, South, West}
impl_block_state_value!{
    CardinalDirection = Self::North;
    Self::North => "north", "north" => Self::North,
    Self::East  => "east",  "east"  => Self::East,
    Self::South => "south", "south" => Self::South,
    Self::West  => "west",  "west"  => Self::West
}

pub enum Direction {North, East, South, West, Up, Down}
impl_block_state_value!{
    Direction = Self::North;
    Self::North => "north", "north" => Self::North,
    Self::East  => "east",  "east"  => Self::East,
    Self::South => "south", "south" => Self::South,
    Self::West  => "west",  "west"  => Self::West,
    Self::Up    => "up",    "up"    => Self::Up,
    Self::Down  => "down",  "down"  => Self::Down
}

pub enum HopperDirection{North, East, South, West, Down}
impl_block_state_value!{
    HopperDirection = Self::North;
    Self::North => "north", "north" => Self::North,
    Self::East  => "east",  "east"  => Self::East,
    Self::South => "south", "south" => Self::South,
    Self::West  => "west",  "west"  => Self::West,
    Self::Down  => "down",  "down"  => Self::Down
}

pub enum BambooLeavesSize {Large, Small, None}
impl_block_state_value!{
    BambooLeavesSize = Self::None;
    Self::Large => "large", "large" => Self::Large,
    Self::Small => "small", "small" => Self::Small,
    Self::None  => "none",  "none"  => Self::None
}

pub enum Rotation16 {
    South, SouthSouthWest, SouthWest,
    WestSouthWest, West, WestNorthWest,
    NorthWest, NorthNorthWest, North, NorthNorthEast, NorthEast,
    EastNorthEast, East, EastSouthEast,
    SouthEast, SouthSouthEast
}
impl_block_state_value!{
    Rotation16 = Self::South;
    Self::South          => "0",  "0"  => Self::South,
    Self::SouthSouthWest => "1",  "1"  => Self::SouthSouthWest,
    Self::SouthWest      => "2",  "2"  => Self::SouthWest,
    Self::WestSouthWest  => "3",  "3"  => Self::WestSouthWest,
    Self::West           => "4",  "4"  => Self::West,
    Self::WestNorthWest  => "5",  "5"  => Self::WestNorthWest,
    Self::NorthWest      => "6",  "6"  => Self::NorthWest,
    Self::NorthNorthWest => "7",  "7"  => Self::NorthNorthWest,
    Self::North          => "8",  "8"  => Self::North,
    Self::NorthNorthEast => "9",  "9"  => Self::NorthNorthEast,
    Self::NorthEast      => "10", "10" => Self::NorthEast,
    Self::EastNorthEast  => "11", "11" => Self::EastNorthEast,
    Self::East           => "12", "12" => Self::East,
    Self::EastSouthEast  => "13", "13" => Self::EastSouthEast,
    Self::SouthEast      => "14", "14" => Self::SouthEast,
    Self::SouthSouthEast => "15", "15" => Self::SouthSouthEast
}

pub enum BedPart {Foot, Head}
impl_block_state_value!{
    BedPart = Self::Foot;
    Self::Foot => "foot", "foot" => Self::Foot,
    Self::Head => "head", "head" => Self::Head
}

macro_rules! define_num {
    ($name:ident, $max:literal $(, $min:literal)?) => (
        pub struct $name(u8);
        impl $name {
            pub fn new(n: u8) -> Self {
                assert!(n<=$max);
                $(assert!(n>=$min);)?
                Self(n)
            }
            pub fn set(&mut self, n: u8) {
                assert!(n<=$max);
                $(assert!(n>=$min);)?
                self.0 = n;
            }
        }
        impl std::ops::Deref for $name {
            type Target = u8;
            fn deref(&self) -> &<Self as std::ops::Deref>::Target {
                &self.0
            }
        }
        impl_block_state_value!{
            $name = 0, u8 => $(min $min)? max $max;
        }
    );
}

define_num!(Num2, 1);
define_num!(Num3, 2);
define_num!(Num4, 3);
define_num!(Num6, 5);
define_num!(Num7, 6);
define_num!(Num8, 7);
define_num!(Num9, 8);
define_num!(Num16, 15);
define_num!(Num26, 25);

define_num!(Count4, 4, 1);
define_num!(Count8, 8, 1);

pub enum BellAttachment {Ceiling, DoubleWall, Floor, SingleWall}
impl_block_state_value!{
    BellAttachment = Self::Floor;
    Self::Ceiling    => "ceiling",     "ceiling"     => Self::Ceiling,
    Self::DoubleWall => "double_wall", "double_wall" => Self::DoubleWall,
    Self::Floor      => "floor",       "floor"       => Self::Floor,
    Self::SingleWall => "single_wall", "single_wall" => Self::SingleWall
}

pub enum Axis {X, Y, Z}
impl_block_state_value!{
    Axis = Self::Y;
    Self::X => "x", "x" => Self::X,
    Self::Y => "y", "y" => Self::Y,
    Self::Z => "z", "z" => Self::Z
}

pub enum CardinalAxis {X, Z}
impl_block_state_value!{
    CardinalAxis = Self::X;
    Self::X => "x", "x" => Self::X,
    Self::Z => "z", "z" => Self::Z
}

pub enum Attachment3 {Ceiling, Floor, Wall}
impl_block_state_value!{
    Attachment3 = Self::Wall;
    Self::Ceiling => "ceiling", "ceiling" => Self::Ceiling,
    Self::Floor   => "floor",   "floor"   => Self::Floor,
    Self::Wall    => "wall",    "wall"    => Self::Wall
}

pub enum ChestType {Left, Right, Single}
impl_block_state_value!{
    ChestType = Self::Single;
    Self::Left   => "left",   "left"   => Self::Left,
    Self::Right  => "right",  "right"  => Self::Right,
    Self::Single => "single", "single" => Self::Single
}

pub enum TallBlockHalf {Lower, Upper}
impl_block_state_value!{
    TallBlockHalf = Self::Lower;
    Self::Lower => "lower", "lower" => Self::Lower,
    Self::Upper => "upper", "upper" => Self::Upper
}

pub enum BlockHalf {Bottom, Top}
impl_block_state_value!{
    BlockHalf = Self::Bottom;
    Self::Bottom => "bottom", "bottom" => Self::Bottom,
    Self::Top    => "top",    "top"    => Self::Top
}

pub enum SlabType {Bottom, Top, Double}
impl_block_state_value!{
    SlabType = Self::Bottom;
    Self::Bottom => "bottom", "bottom" => Self::Bottom,
    Self::Top    => "top",    "top"    => Self::Top,
    Self::Double => "double", "double" => Self::Double
}

pub enum StairShape {
    InnerLeft,
    InnerRight,
    OuterLeft,
    OuterRight,
    Straight
}
impl_block_state_value!{
    StairShape = Self::Straight;
    Self::InnerLeft => "inner_left", "inner_left" => Self::InnerLeft,
    Self::InnerRight => "inner_right", "inner_right" => Self::InnerRight,
    Self::OuterLeft => "outer_left", "outer_left" => Self::OuterLeft,
    Self::OuterRight => "outer_right", "outer_right" => Self::OuterRight,
    Self::Straight => "straight", "straight" => Self::Straight
}

pub enum Side {Left, Right}
impl_block_state_value!{
    Side = Self::Left;
    Self::Left  => "left",  "left"  => Self::Left,
    Self::Right => "right", "right" => Self::Right
}

pub enum Instrument {
    Banjo,
    Basedrum,
    Bass,
    Bell,
    Bit,
    Chime,
    CowBell,
    Didgeridoo,
    Flute,
    Guitar,
    Harp,
    Hat,
    IronXylophone,
    Pling,
    Snare,
    Xylophone
}
impl_block_state_value!{
    Instrument = Self::Harp;
    Self::Banjo         => "banjo",         "banjo"         => Self::Banjo,
    Self::Basedrum      => "basedrum",      "basedrum"      => Self::Basedrum,
    Self::Bass          => "bass",          "bass"          => Self::Bass,
    Self::Bell          => "bell",          "bell"          => Self::Bell,
    Self::Bit           => "bit",           "bit"           => Self::Bit,
    Self::Chime         => "chime",         "chime"         => Self::Chime,
    Self::CowBell       => "cowbell",       "cowbell"       => Self::CowBell,
    Self::Didgeridoo    => "didgeridoo",    "didgeridoo"    => Self::Didgeridoo,
    Self::Flute         => "flute",         "flute"         => Self::Flute,
    Self::Guitar        => "guitar",        "guitar"        => Self::Guitar,
    Self::Harp          => "harp",          "harp"          => Self::Harp,
    Self::Hat           => "hat",           "hat"           => Self::Hat,
    Self::IronXylophone => "ironxylophone", "ironxylophone" => Self::IronXylophone,
    Self::Pling         => "pling",         "pling"         => Self::Pling,
    Self::Snare         => "snare",         "snare"         => Self::Snare,
    Self::Xylophone     => "xylophone",     "xylophone"     => Self::Xylophone
}

pub enum PistonType {Normal, Sticky}
impl_block_state_value!{
    PistonType = Self::Normal;
    Self::Normal => "normal", "normal" => Self::Normal,
    Self::Sticky => "sticky", "sticky" => Self::Sticky
}

pub enum RailShape {
    EastWest,
    NorthEast,
    NorthSouth,
    NorthWest,
    SouthEast,
    SouthWest,
    AscendingEast,
    AscendingNorth,
    AscendingSouth,
    AscendingWest
}
impl_block_state_value!{
    RailShape = Self::NorthSouth;
    Self::EastWest       => "east_west",       "east_west"       => Self::EastWest,
    Self::NorthEast      => "north_east",      "north_east"      => Self::NorthEast,
    Self::NorthSouth     => "north_south",     "north_south"     => Self::NorthSouth,
    Self::NorthWest      => "north_west",      "north_west"      => Self::NorthWest,
    Self::SouthEast      => "south_east",      "south_east"      => Self::SouthEast,
    Self::SouthWest      => "south_west",      "south_west"      => Self::SouthWest,
    Self::AscendingEast  => "ascending_east",  "ascending_east"  => Self::AscendingEast,
    Self::AscendingNorth => "ascending_north", "ascending_north" => Self::AscendingNorth,
    Self::AscendingSouth => "ascending_south", "ascending_south" => Self::AscendingSouth,
    Self::AscendingWest  => "ascending_west",  "ascending_west"  => Self::AscendingWest
}

pub enum StraightRailShape {
    EastWest,
    NorthSouth,
    AscendingEast,
    AscendingNorth,
    AscendingSouth,
    AscendingWest
}
impl_block_state_value!{
    StraightRailShape = Self::NorthSouth;
    Self::EastWest       => "east_west",       "east_west"       => Self::EastWest,
    Self::NorthSouth     => "north_south",     "north_south"     => Self::NorthSouth,
    Self::AscendingEast  => "ascending_east",  "ascending_east"  => Self::AscendingEast,
    Self::AscendingNorth => "ascending_north", "ascending_north" => Self::AscendingNorth,
    Self::AscendingSouth => "ascending_south", "ascending_south" => Self::AscendingSouth,
    Self::AscendingWest  => "ascending_west",  "ascending_west"  => Self::AscendingWest
}

pub enum ComparatorMode {Compare, Subtract}
impl_block_state_value!{
    ComparatorMode = Self::Compare;
    Self::Compare => "compare", "compare" => Self::Compare,
    Self::Subtract => "subtract", "subtract" => Self::Subtract
}

pub enum DustConnection {None, Side, Up}
impl_block_state_value!{
    DustConnection = Self::None;
    Self::None => "none", "none" => Self::None,
    Self::Side => "side", "side" => Self::Side,
    Self::Up   => "up",   "up"   => Self::Up
}

pub enum StructureBlockMode {
    Corner,
    Data,
    Load,
    Save
}
impl_block_state_value!{
    StructureBlockMode = Self::Data;
    Self::Corner => "corner", "corner" => Self::Corner,
    Self::Data => "data", "data" => Self::Data,
    Self::Load => "load", "load" => Self::Load,
    Self::Save => "save", "save" => Self::Save
}
