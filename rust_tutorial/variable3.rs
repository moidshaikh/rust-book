struct Greeting {
 name: String,
}

// Constructor for the struct. static method
impl Greeting {
 fn new(name: &str) -> Self {
  Greeting {
   name: name.to_string(),
  }
 }
}

fn main() {
 let greeting = Greeting::new("Moid");
 println!("Hola, {}!", greeting.name);
}
