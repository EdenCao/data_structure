use std::cmp::Ordering;

fn main() {
  let arr = [0, 12, 56, 45];
  println!("{:?}", binary_search(&arr, &56));
}

fn binary_search(num_list: &[i32], num: &i32) -> (usize, i32) {
  let mut lo = 0;
  let mut hi = num_list.len() - 1;
  let mut mid = (lo + hi) / 2;

  while(lo <= hi) {
    mid = (lo + hi) / 2;

    match num_list[mid].cmp(num) {
      Ordering::Greater => {
        hi = mid;
      },
      Ordering::Equal => {
        return (mid, num_list[mid]);
      },
      Ordering::Less => {
        lo = mid
      }
    }
  }

  return (0, -1)
}
