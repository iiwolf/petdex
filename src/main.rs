struct Pet{
    name: String,
    age: f32,
}

impl std::fmt::Display for Pet{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        write!(f, "{}, Age {:0.1}", self.name, self.age)
    }
}
fn main() {
    println!("Petdex");

    let pet = Pet{name: "Arley".to_string(), age: 4.5};
    println!("{}", pet);
}
