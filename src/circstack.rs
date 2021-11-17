/// The CircStack, along with the stage, is a central data
/// storage element in the Riddle language
pub struct CircStack<T> {
  /// The elements used in the CircStack
  elems: Vec<T>,
}

impl<T> CircStack<T> {
  /// create a new empty CircStack
  pub fn new() -> Self {
    CircStack { elems : Vec::new() }
  }

  /// push a value onto the CircStack
  pub fn push(&mut self, val: T) {
    self.elems.push(val)
  }

  /// pop a value off the CircStack if it has any
  pub fn pop(&mut self) -> Option<T> {
    self.elems.pop()
  }

  /// peek function looks at top elem, for convenience
  pub fn peek(&self) -> Option<&T> {
    self.elems.last()
  }

  /// check if any data is stored currently
  pub fn is_empty(&self) -> bool {
    self.elems.is_empty()
  }

  /// check how many elements are currently stored
  pub fn len(&self) -> usize {
    self.elems.len()
  }

  /// swap the top two elements of the stack
  pub fn swap(&mut self) {
    let top    = self.elems.pop();
    let subtop = self.elems.pop();

    if top.is_some() {
      if subtop.is_some() {
        self.elems.push(top.unwrap());
        self.elems.push(subtop.unwrap());
      }
      else {
        self.elems.push(top.unwrap());
      }
    }
  }

  /// apply and arithmetic or logical operation
  /// to the top element of a CircStack
  pub fn apply_one(&mut self, func: fn(T) -> T) {
    let top    = self.elems.pop();

    if top.is_some() {
      self.elems.push(func(top.unwrap()));
    }
  }

  /// apply an arithemtic or logic operation to the
  /// top two elements of a CircStack
  pub fn apply_two(&mut self, func: fn(T, T) -> T) {
    let top    = self.elems.pop();
    let subtop = self.elems.pop();

    if top.is_some() {
      if subtop.is_some() {
        self.elems.push(func(top.unwrap(), subtop.unwrap()));
      }
      else {
        self.elems.push(top.unwrap());
      }
    }
  }

  /// one of the "circular" functions
  /// pop the top and shove it underneath
  pub fn rotate_from_top(&mut self) {
    let top = self.elems.pop();

    if top.is_some() {
      self.elems.reverse(); // flip rest of stack
      self.elems.push(top.unwrap()); // push old top
      self.elems.reverse(); // flip again and it's the bottom
    }
  }

  /// The other "circular" function
  /// slip the bottom element from the stack out
  /// and place it on the top
  pub fn rotate_from_bottom(&mut self) {
    self.elems.reverse(); // flip stack
    let bottom = self.elems.pop(); // grab the bottom

    if bottom.is_some() {
      self.elems.reverse(); // flip back again
      self.elems.push(bottom.unwrap()); // bottom now on top
    }
  }

}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_circstack_empty() {
    let cstack: CircStack<isize> = CircStack::new();

    assert_eq!(cstack.is_empty(), true);
    assert_eq!(cstack.len(), 0);
    assert_eq!(cstack.peek(), None);    
  }

  #[test]
  #[should_panic]
  fn test_circstack_peek_panic() {
    let cstack: CircStack<isize> = CircStack::new();
    cstack.peek().unwrap(); // force unwrap a None generates panic   
  }

  #[test]
  #[should_panic]
  fn test_circstack_pop_panic() {
    let mut cstack: CircStack<isize> = CircStack::new();
    cstack.pop().unwrap(); // force unwrap a None generates panic   
  }

  #[test]
  fn test_circstack_push_1() {
    let mut cstack: CircStack<isize> = CircStack::new();
    let push_val = 12; // TODO: find way to make random isize
    cstack.push(push_val);

    assert_eq!(cstack.is_empty(), false);
    assert_eq!(cstack.len(), 1);
    assert_eq!(*cstack.peek().unwrap(), push_val);
  }

  #[test]
  fn test_circstack_pop_1() {
    let mut cstack: CircStack<isize> = CircStack::new();
    let test_val = 7; // TODO: find way to make random isize
    cstack.push(test_val);

    assert_eq!(cstack.pop().unwrap(), test_val);
    assert_eq!(cstack.is_empty(), true);
  }

  #[test]
  fn test_circstack_swap_2() {
    let mut cstack: CircStack<isize> = CircStack::new();

    let val_0 = 7;  // TODO: find way to make random isize
    let val_1 = 13; // TODO: find way to make DIFFERENT random isize
    cstack.push(val_0);
    cstack.push(val_1);

    cstack.swap();

    assert_eq!(cstack.pop().unwrap(), val_0);
    assert_eq!(cstack.pop().unwrap(), val_1);
  }

  #[test]
  fn test_circstack_add() {
    let mut cstack: CircStack<isize> = CircStack::new();

    let val_0 = 43; // TODO: find way to make random isize
    let val_1 = 22; // TODO: find way to make DIFFERENT random isize
    cstack.push(val_0);
    cstack.push(val_1);

    cstack.apply_two(|x, y| x + y);

    assert_eq!(cstack.pop().unwrap(), val_0 + val_1);
  }

  #[test]
  fn test_circstack_sub() {
    let mut cstack: CircStack<isize> = CircStack::new();

    let val_0 = 43; // TODO: find way to make random isize
    let val_1 = 22; // TODO: find way to make DIFFERENT random isize
    cstack.push(val_0);
    cstack.push(val_1);

    cstack.apply_two(|x, y| x - y);

    assert_eq!(cstack.pop().unwrap(), val_1 - val_0);
  }

  #[test]
  fn test_circstack_mul() {
    let mut cstack: CircStack<isize> = CircStack::new();

    let val_0 = 43; // TODO: find way to make random isize
    let val_1 = 22; // TODO: find way to make DIFFERENT random isize
    cstack.push(val_0);
    cstack.push(val_1);

    cstack.apply_two(|x, y| x * y);

    assert_eq!(cstack.pop().unwrap(), val_0 * val_1);
  }

  #[test]
  fn test_circstack_div() {
    let mut cstack: CircStack<isize> = CircStack::new();

    let val_0 = 43; // TODO: find way to make random isize
    let val_1 = 22; // TODO: find way to make DIFFERENT random isize
    cstack.push(val_0);
    cstack.push(val_1);

    cstack.apply_two(|x, y| x / y);

    assert_eq!(cstack.pop().unwrap(), val_1 / val_0);
  }

  #[test]
  fn test_circstack_not() {
    let mut cstack: CircStack<isize> = CircStack::new();

    let val_0 = 51; // TODO: find way to make random isize
    cstack.push(val_0);

    cstack.apply_one(|x| !x);

    assert_eq!(cstack.pop().unwrap(), !val_0);
  }

  #[test]
  fn test_circstack_top_rotate() {
    let mut cstack: CircStack<isize> = CircStack::new();

    let val_0 = 51; // TODO: find way to make random isize
    let val_1 = -22; // TODO: rand array, list?
    let val_2 = 0;
    let val_3 = 13;
    cstack.push(val_0);
    cstack.push(val_1);
    cstack.push(val_2);
    cstack.push(val_3);

    cstack.rotate_from_top(); 

    assert_eq!(cstack.pop().unwrap(), val_2);
    assert_eq!(cstack.pop().unwrap(), val_1);
    assert_eq!(cstack.pop().unwrap(), val_0);
    assert_eq!(cstack.pop().unwrap(), val_3);
  }

  #[test]
  fn test_circstack_bottom_rotate() {
    let mut cstack: CircStack<isize> = CircStack::new();

    let val_0 = -151; // TODO: find way to make random isize
    let val_1 = 222; // TODO: rand array, list?
    let val_2 = 1;
    let val_3 = -40;
    cstack.push(val_0);
    cstack.push(val_1);
    cstack.push(val_2);
    cstack.push(val_3);

    cstack.rotate_from_bottom(); 

    assert_eq!(cstack.pop().unwrap(), val_0);
    assert_eq!(cstack.pop().unwrap(), val_3);
    assert_eq!(cstack.pop().unwrap(), val_2);
    assert_eq!(cstack.pop().unwrap(), val_1);
  }

}
