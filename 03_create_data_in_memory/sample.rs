struct SeaCreature {
  animal_type: String,
  name: String,
  arms: i32,
  legs: i32,
  weapon: String,
}

struct Location(i32, i32);

struct Marker;

fn main() {
  // create data in memory
  let ferris = SeaCreature {
    animal_type: String::from("crab"),
    name: String::from("Ferris"),
    arms: 2,
    legs: 4,
    weapon: String::from("claw"),
  };

  let sarah = SeaCreature {
    animal_type: String::from("octopus"),
    name: String::from("Sarah"),
    arms: 8,
    legs: 0,
    weapon: String::from("none"),
  };

  println!(
    "{} is a {}. They have {} arms, {} legs, and {} weapon",
    ferris.name, ferris.animal_type, ferris.arms, ferris.legs, ferris.weapon
  );

  println!(
    "{} is a {}. They have {} arms, {} legs, and {} weapon",
    sarah.name, sarah.animal_type, sarah.arms, sarah.legs, sarah.weapon
  );

  // taple-like structs
  let loc = Location(42, 32);
  println!("{}, {}", loc.0, loc.1);

  // unit-like structs
  let _m = Marker;
}
