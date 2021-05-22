mod shared
use std::env::current_dir;
use shared::is_home

fn main() -> Result<(), u32> {
  let c_dir = current_dir().unwrap();
  let dirs = c_dir.to_str().unwrap().split("/").collect::<Vec<&str>>();
  let mut dir_len = dirs.len() - 1;
  // /Users/<uname> on macOS, /home/<uname> on Linux.
  if is_home(&dirs[1]) && dir_len >= 2 {
    dir_len -= 2
  }
  if dir_len >= 2 {
    return Err(1)
  }
  Ok(())
}