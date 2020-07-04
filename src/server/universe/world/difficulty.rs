#[derive(Clone, Debug, Copy)]
pub enum Difficulty {
  Peaceful = 0,
  Easy = 1,
  Normal = 2,
  Hard = 3,
}

impl std::fmt::Display for Difficulty {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
    match self {
      Difficulty::Peaceful => write!(f, "Difficulty::Peaceful"),
      Difficulty::Easy => write!(f, "Difficulty::Easy"),
      Difficulty::Normal => write!(f, "Difficulty::Normal"),
      Difficulty::Hard => write!(f, "Difficulty::Hard"),
    }
  }
}
