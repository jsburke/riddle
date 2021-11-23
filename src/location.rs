/// The direction enum is used to both indicate how
/// actors move and face in Riddle
enum Direction {
  Up,
  Down,
  Left,
  Right,
}

/// Coords describes the location of actors in a given
/// stage in Riddle
pub struct Coords {
  x: usize,
  y: usize
}

impl Coords {
  pub fn new() -> Self {
    Coords { x : 0, y : 0 }
  }
}
