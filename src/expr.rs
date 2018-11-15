use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();
  let expr: Vec<char> = args[1].chars().collect();

  let mut ops: Vec<char> = Vec::new();
  let mut vals: Vec<u32> = Vec::new();
  println!("expr = {}", args[1]);
  for ch in expr.into_iter() {
    if ch == '(' {
      // do nothing
    } else if ch == '+' {
      ops.push(ch);
    } else if ch == '-' {
      ops.push(ch);
    } else if ch == '*' {
      ops.push(ch);
    } else if ch == '/' {
      ops.push(ch);
    } else if ch == ')' {
      println!("{:?}", ops);
      println!("{:?}", vals);
      let op = ops.pop().unwrap();
      let mut v = vals.pop().unwrap();
      if op == '+' {
        v = vals.pop().unwrap() + v;
      } else if op == '-' {
        v = vals.pop().unwrap() - v;
      } else if op == '*' {
        v = vals.pop().unwrap() * v;
      } else if op == '/' {
        v = vals.pop().unwrap() / v;
      }
      vals.push(v);
    } else {
      vals.push(ch.to_digit(10).unwrap());
    }
  }

  println!("{}", vals.pop().unwrap());
}
