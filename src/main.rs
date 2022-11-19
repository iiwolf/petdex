trait Pet{
    fn name(&self) -> &'static str;
    fn age(&self) -> f32;
}

struct Dog{name: &'static str, age: f32}

impl Pet for Dog{
    fn name(&self) -> &'static str {self.name}
    fn age(&self) -> f32 {self.age}
}

impl std::fmt::Display for dyn Pet{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        write!(f, "{}, Age {:0.1}", self.name(), self.age())
    }
}
fn main() {
    println!("Petdex");

    let pet = Dog{name: "Arley", age: 4.5};
    println!("{}", pet);
}
