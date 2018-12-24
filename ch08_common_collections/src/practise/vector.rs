pub fn find_mean(vec: &Vec<i32>) -> Option<f32> {
  let len = vec.len();
  if len == 0 {
    return None;
  } else {
    let sum: f32 = vec.iter().map(|&v| v as f32).sum();
    Some(sum / len as f32)
  }
}

pub fn find_median(vec: &Vec<i32>) -> Option<f64> {
  let len = vec.len();
  if len == 0 {
    return None;
  }

  let is_odd: bool = len % 2 == 1;

  match is_odd {
    true => Some(vec[len / 2] as f64),
    false => {
      let median = len / 2;
      return Some((vec[median] + vec[median - 1]) as f64 / 2.0);
    }
  }
}
