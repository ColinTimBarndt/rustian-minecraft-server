#[derive(Clone, Debug, Copy)]
pub enum Dimension {
    Nether = -1,
    Overworld = 0,
    End = 1,
}

impl std::fmt::Display for Dimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Dimension::Nether => write!(f, "Dimension::Nether"),
            Dimension::Overworld => write!(f, "Dimension::Overworld"),
            Dimension::End => write!(f, "Dimension::End"),
        }
    }
}
