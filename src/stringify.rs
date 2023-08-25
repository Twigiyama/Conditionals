fn main() {
    let mut literal = String::from("Earth");
    println!("literal is {literal}");
    literal.push_str(" is home");
    println!("literal is now {literal}");
    
    let outer_planet: String;

    {   let  inner_planet = String::from("Mercury");
        println!("inner planet is {inner_planet}");
        outer_planet = inner_planet.clone();
        println!("Inner planet might be {inner_planet}");
    }
    println!("The outer planet is {outer_planet}");

    let mut rocket_fuel = produce_fuel();
    let length = process_fuel(&mut rocket_fuel);
    println!("Rocket fuel is {rocket_fuel} and length is {length}");

    fn process_fuel(propellant: &mut String) -> usize {
        propellant.clear();
        propellant.push_str("Propane");
        println!("Processing propellant {propellant}");
        let length = propellant.len();
        return length;
    }

    fn produce_fuel() -> String {
        let new_fuel = String::from("RP-1");
        new_fuel
    }

    let message = String::from("Greetings from Earth!");
    println!("message is {message}");

    let last_word = &message[15..15+5];
    println!("last word is {last_word}");

    let planets = [1,2,3,4,5,6,7,8];
    let inner_planets: &[i32] = &planets[..4];
    print!("inner planets are {:?}", inner_planets);


    let new_message = String::from("Greetings from the human occupied planet");
    let first_word = get_first_word(&new_message[10..]);
    println!("first word is {first_word}");

    fn get_first_word(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (index, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[..index];
            }
        }

        &s
    }

}
