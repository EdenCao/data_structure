
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn file_read_by_line<T>(path: &str) -> Vec<T>
where
  T: std::str::FromStr + std::clone::Clone,
  T::Err: std::fmt::Debug
{
  let file = File::open(path).expect("read file error");
  println!("read file: {}", path);

  let fin = BufReader::new(file).lines();
  let mut data_queue: Vec<T> = [].to_vec();

  for data in fin {
    data_queue.push(data.unwrap().parse::<T>().unwrap());
  }
  
  println!("read all numbers, total: {}", data_queue.len());

  return data_queue;
}
