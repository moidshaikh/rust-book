use std::fmt;

struct Greeting {
 name: String,
}

impl Greeting {
 fn new<T: AsRef<str>>(name: T) -> Self {
  Greeting {
   name: name.as_ref().to_string(),
  }
 }
}

// using new func to print greeting instead of directly printing it
impl fmt::Display for Greeting {
 fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
  write!(f, "Helllooo, {}!", self.name)
 }
}


fn main() {
 let greeting = Greeting::new("Moid");
 println!("Hola, {}!", greeting);
}
