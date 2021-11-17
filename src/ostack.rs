/// The ostack, along with the stage, is a central data
/// storage element in the Riddle language
pub struct ostack<T> {
  /// The elements used in the ostack
  elems: Vec<T>,
}

impl<T> ostack<T> {
  /// create a new empty ostack
  pub fn new() -> Self {
    ostack { elems : Vec::new() }
  }

  /// push a value onto the ostack
  pub fn push(&mut self, val: T) {
    self.elems.push(val)
  }

  /// pop a value off the ostack if it has any
  pub fn pop(&mut self) -> Option<T> {
    self.elems.pop()
  }

  /// check if any data is stored currently
  pub fn is_empty(&mut self) -> bool {
    self.elems.is_empty()
  }

  /// check how many elements are currently stored
  pub fn len(&mut self) -> usize {
    self.elems.len()
  }

  /// swap the top two elements of the stack
  pub fn swap(&mut self) {
    if self.elems.len() > 1 {
      let top    = self.elems.pop();
      let subtop = self.elems.pop();
   
      self.elems.push(top);
      self.elems.push(subtop);
    }
  }

  /// apply an arithemtic or logic operation to the
  /// top two elements of an ostack
  pub fn fn_apply(&mut self, func: fn(T, T) -> T) {
    if self.elems.len() > 1 {
      let top    = self.elems.pop();
      let subtop = self.elems.pop();

      self.elems.push(func(top, subtop))
    }
  }
}
