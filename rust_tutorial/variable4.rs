struct Greeting {
 name: String,
}

// Constructor for the struct. static method
// not using string. 
impl Greeting {
 fn new<T: AsRef<str>>(name: T) -> Self {
  Greeting {
   name: name.as_ref().to_string(),
  }
 }
}

fn main() {
 let greeting = Greeting::new("Moid");
 println!("Hola, {}!", greeting.name);
}
