use rand::prelude::*;
// pub trait Pet{
//     fn name(&self) -> &'static str;
//     fn age(&self) -> f32;

//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
//         write!(f, "{}, Age {:0.1}", self.name(), self.age())
//     }

//     fn get_info_string(&self) -> String {
//         format!("{}, Age {:0.1}", self.name(), self.age())
//     }
// }

// struct Dog{name: &'static str, age: f32}

// impl Pet for Dog{
//     fn name(&self) -> &'static str {self.name}
//     fn age(&self) -> f32 {self.age}
// }

// impl std::fmt::Display for dyn Pet{
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
//         write!(f, "{}, Age {:0.1}", self.name(), self.age())
//     }
// }

use std::default;

struct Medication{}
struct Ailment{}

#[derive(Default)]
enum Species {
    Dog,
    Cat,
    Bird,
    #[default]
    Unknown
}

#[derive(Default)]
struct Pet{
    name: &'static str,
    age: f32,
    species: Species,
    ailments: Vec<Ailment>,
    medications: Vec<Medication>,
    nicknames: Vec<&'static str>,
    adjectives: Vec<&'static str>,
    status: &'static str,
}

impl std::fmt::Display for Pet{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        write!(f, "{}, Age {:0.1}", self.name, self.age)
    }
}

impl Pet{
    fn add_nickname(&mut self, name: &'static str){
        self.nicknames.push(name)
    }
    
    fn add_adjective(&mut self, name: &'static str){
        self.adjectives.push(name)
    }

    fn random_nickname(&self) -> String {
        format!("{} {}", 
            self.adjectives.choose(&mut rand::thread_rng()).unwrap(),
            self.nicknames.choose(&mut rand::thread_rng()).unwrap()
        )
    }

    fn display_string(&self) -> String {
        format!("Look at that {}", self.random_nickname())
    }
}
fn main() {
    println!("\n\n~~~~ Petdex ~~~~\n");

    let pets: Vec<Pet> = vec![
        Pet{
            name: "Arley", 
            age: 4.5, 
            species: Species::Dog, 
            nicknames: vec!["POOPER", "bug", "snugglebuns"],
            adjectives: vec!["stinky", "heckin'", "tiny"],
            ..Default::default()
        },

        Pet{
            name: "Milo", 
            age: 10.0, 
            species: Species::Dog, 
            nicknames: vec!["Moosh", "Mr. Moosh"],
            adjectives: vec!["big", "heckin'"],
            ..Default::default()
        },

        Pet{
            name: "Charlie", 
            age: 10.0, 
            species: Species::Dog, 
            nicknames: vec!["Cheese", "Charles", "Char-Char"],
            adjectives: vec!["floofy", "lil'"],
            ..Default::default()
        }
    ];
    
    // pet.get_info_string();
    for pet in pets{
        println!("{}: {}", pet, pet.display_string());
    }


}
