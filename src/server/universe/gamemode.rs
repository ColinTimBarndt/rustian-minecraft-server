#[derive(Clone, Debug, Copy)]
pub enum Gamemode {
    Survival = 0,
    Creative = 1,
    Adventure = 2,
    Spectator = 3,
}

impl std::fmt::Display for Gamemode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Gamemode::Survival => write!(f, "Survival"),
            Gamemode::Creative => write!(f, "Creative"),
            Gamemode::Adventure => write!(f, "Adventure"),
            Gamemode::Spectator => write!(f, "Spectator"),
        }
    }
}
