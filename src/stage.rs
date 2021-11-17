/// The Stage, along with the CircStack, is a central data
/// storage element in the Riddle language. The Stage is
/// the internal representation of the consumed source
/// file that may be actively modified during evalutation
pub struct Stage {
  /// The elements used in the CircStack
  num_rows: usize,
  num_cols: usize,
  tiles:    [[u8; num_rows]; num_cols],
}

