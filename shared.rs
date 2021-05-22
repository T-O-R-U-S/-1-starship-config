// only used this to make rustc shut up about "no main!"
fn main(){
  panic!("Not supposed to be used individually!")
}

pub fn is_home(string: &str) -> bool {
  string == "home" || string == "Users"
}