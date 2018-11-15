struct CapacityStack {
  str_stack: Vec<String>,
  len: u32
}

impl CapacityStack {
  pub fn new() -> CapacityStack {
    CapacityStack {
      str_stack: Vec::new(),
      len: 0
    }
  }

  pub fn push(&mut self, s: String) {
    self.str_stack.push(s);
    self.len += 1;
  }

  pub fn pop(&mut self) -> String {
    self.len -= 1;
    self.str_stack.pop().unwrap()
  }

  pub fn is_empty(&self) -> bool {
    match self.str_stack.len() {
      0 => true,
      _ => false
    }
  }

  pub fn size(&self) -> u32 {
    self.len
  }
}

fn main() {
  let mut s_stack = CapacityStack::new();
  let s1 = String::from("just test");
  s_stack.push(s1);
  s_stack.push(String::from("test2"));
  assert_eq!(s_stack.size(), 2);
  assert_eq!(s_stack.is_empty(), false);
  let top = s_stack.pop();
  assert_eq!(top, "test2");
  assert_eq!(s_stack.size(), 1);
  s_stack.pop();
  assert_eq!(s_stack.size(), 0);
  assert_eq!(s_stack.is_empty(), true);
}