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
}