use std::env::current_dir;

fn main() -> Result<(), u32> {
  let c_dir = current_dir().unwrap();
  let dirs = c_dir.to_str().unwrap().split("/").collect::<Vec<&str>>();
  let mut dir_len = dirs.len() - 1;
  if dirs[1] == "Users" {
    dir_len -= 2
  }
  if dir_len > 2 {
    return Err(1)
  }
  Ok(())
}