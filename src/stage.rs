/// The Stage, along with the CircStack, is a central data
/// storage element in the Riddle language. The Stage is
/// the internal representation of the consumed source
/// file that may be actively modified during evalutation
pub struct Stage {
  /// dimenstions of the stage
  num_rows: usize,
  num_cols: usize,
  /// characters from the file, padded with spaces
  tiles:    Vec<Vec<u8>>
}

