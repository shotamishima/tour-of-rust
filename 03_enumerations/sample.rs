#![allow(dead_code)]

enum Species { Crab, Octopus, Fish, Clam }
enum PoisonType { Acidic, Painful, Lethal }
enum Size { Big, Small}
enum Weapon {
  Claw(i32, Size),
  Poison(PoisonType),
  None
}

struct SeaCreature {
  species: Species,
  name: String,
  arms: i32,
  legs: i32,
  weapon: Weapon,
}

fn main() {
  let ferris = SeaCreature {
    species: Species::Crab,
    name: String::from("Ferris"),
    arms: 2,
    legs: 4,
    weapon: Weapon::Claw(2, Size::Small),
};

  // Enumerations
  // match ferris.species {
  //   Species::Crab => println!("{} is a crab", ferris.name),
  //   Species::Octopus => println!("{} is a octopus", ferris.name),
  //   Species::Fish => println!("{} is a fish", ferris.name),
  //   Species::Clam => println!("{} is a clam", ferris.name),
  // }

  // Enumerations with type
  match ferris.species {
    Species::Crab => {
      match ferris.weapon {
        Weapon::Claw(num_claws, size) => {
          let size_description = match size {
            Size::Big => "big",
            Size::Small => "small",
          };
          println!("ferris is a crab with {} {} claws", num_claws, size_description)
        },
        _ => println!("ferris is a crab with some other weapon")
      }
    },
    _ => println!("ferris is some other animal"),
  }
} 
