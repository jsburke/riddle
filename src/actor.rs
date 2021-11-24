use crate::circstack;

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
type Coord = (u8, u8);

/// Actors in Riddle are elements that operate within
/// a given stage with one being player controllable,
/// and others being automata in Riddle that can alter
/// execution or play
pub struct Actor {
  /// location in the current Stage
  coords:    Coord,

  /// which way is the actor facing
  dir:       Direction,

  is_player: bool,

  /// if player, current stacks
  stacks:    Vec<*mut circstack::CircStack<isize>>,
}
