// Codesafety, thread safety, memory safety, type safety, no null pointers, no dangling pointers, no buffer overflows, no uninitialized
// System language, for Systems not applications
// no garbage collection in rust, rust checks when needed and frees memory when needed
// Rust compiler is strict, it will not compile if there are any errors
// cargo - package manager and build system, dev and release build
// rustc - rust compiler
// good backward compatibility, rust code will never break

fn greet(name: String) -> String {
  return format!("Hello, {}", name);
}

fn add_two_numbers(a: i32, b: i32) -> i32 {
  return a + b;
}

fn binary_search(arr: Vec<i32>, target: i32) -> i32 {
  let mut left: i32 = 0;
  let mut right: i32 = arr.len() as i32 - 1;
  while left <= right {
    let mid: i32 = left + (right - left) / 2;
    if arr[mid as usize] == target {
      return mid;
    } else if arr[mid as usize] < target {
      left = mid + 1;
    } else {
      right = mid - 1;
    }
  }
  return -1;
}

fn sum_of_array(arr: Vec<i32>) -> i32 {
  let mut sum: i32 = 0;
  for i in 0..arr.len() {
    sum += arr[i];
  }
  return sum;
}

fn find_max(arr: Vec<i32>) -> i32 {
  let mut max: i32 = arr[0];
  for i in 1..arr.len() {
    if arr[i] > max {
      max = arr[i];
    }
  }
  return max;
}

fn main(){
    println!("Hello, World!");
    let name: &str = "Rust";
    greet(name.to_string());
    let a: i32 = 10;
    let b: i32 = 20;
    println!("Sum of {} and {} is {}", a, b, add_two_numbers(a, b));
    let array: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let target: i32 = 5;
    println!("Index of {} in array is {}", target, binary_search(array.clone(), target));
    println!("Sum of array is {}", sum_of_array(array.clone()));
    println!("Max element in array is {}", find_max(array.clone()));
}

