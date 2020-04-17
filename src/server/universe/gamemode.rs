#[derive(Clone, Debug, Copy)]
pub enum Gamemode {
    Survival  = 0,
    Creative  = 1,
    Adventure = 2,
    Spectator = 3,
}

impl std::fmt::Display for Gamemode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Gamemode::Survival  => write!(f, "Gamemode::Survaval"),
            Gamemode::Creative  => write!(f, "Gamemode::Creative"),
            Gamemode::Adventure => write!(f, "Gamemode::Adventure"),
            Gamemode::Spectator => write!(f, "Gamemode::Spectator")
        }
    }
}