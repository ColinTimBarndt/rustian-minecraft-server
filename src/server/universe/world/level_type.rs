const DEFAULT:      &str = "default";
const FLAT:         &str = "flat";
const LARGE_BIOMES: &str = "largeBiomes";
const AMPLIFIED:    &str = "amplified";
const CUSTOMIZED:   &str = "customized";
const BUFFET:       &str = "buffet";
const DEFAULT_1_1:  &str = "default_1_1";

#[derive(Clone, Copy, Debug)]
pub enum LevelType {
    Default,
    Flat,
    LargeBiomes,
    Amplified,
    Customized,
    Buffet,
    Default1_1
}
impl LevelType {
    pub fn to_string(self) -> String {
        match self {
            LevelType::Default     => DEFAULT.to_string(),
            LevelType::Flat        => FLAT.to_string(),
            LevelType::LargeBiomes => LARGE_BIOMES.to_string(),
            LevelType::Amplified   => AMPLIFIED.to_string(),
            LevelType::Customized  => CUSTOMIZED.to_string(),
            LevelType::Buffet      => BUFFET.to_string(),
            LevelType::Default1_1  => DEFAULT_1_1.to_string()
        }
    }
}
impl std::fmt::Display for LevelType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            LevelType::Default     => write!(f, "LevelType::Default"),
            LevelType::Flat        => write!(f, "LevelType::Flat"),
            LevelType::LargeBiomes => write!(f, "LevelType::LargeBiomes"),
            LevelType::Amplified   => write!(f, "LevelType::Amplified"),
            LevelType::Customized  => write!(f, "LevelType::Customized"),
            LevelType::Buffet      => write!(f, "LevelType::Buffet"),
            LevelType::Default1_1  => write!(f, "LevelType::Default1_1")
        }
    }
}