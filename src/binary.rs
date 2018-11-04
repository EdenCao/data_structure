// extern crate elapsed;

// use elapsed::measure_time;
use std::cmp::Ordering;
use std::env;

mod file_reader;

fn main() {
  let args: Vec<String> = env::args().collect();
  let file_name = &args[1];
  let key = &args[2];

  let nums = file_reader::file_read_by_line(&file_name);

  println!("read all numbers, total: {}", nums.len());

  let (position, key) = binary_search(&nums, &key.parse::<i32>().unwrap());

  if key == -1 {
    println!("not found");
  } else {
    println!("found position: {}, key: {}", position, key);
  }
}

fn binary_search(num_list: &[i32], num: &i32) -> (usize, i32) {
  let mut lo = 0;
  let mut hi = num_list.len() - 1;
  let mut mid = (lo + hi) / 2;

  while (lo <= hi) {
    mid = (lo + hi) / 2;

    match num_list[mid].cmp(num) {
      Ordering::Greater => {
        hi = mid;
      }
      Ordering::Equal => {
        return (mid, num_list[mid]);
      }
      Ordering::Less => lo = mid,
    }
  }

  return (0, -1);
}
