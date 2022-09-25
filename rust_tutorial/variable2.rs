// creating own variable type

struct Greeting {
 name: String,
}


fn main() {
 let greeting = Greeting { name: "Moid".to_string() };
 println!("Hallo, {}!", greeting.name);

}
