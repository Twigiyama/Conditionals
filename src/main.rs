fn main() {
    let make_x_odd = true;
    let x: i32 = if make_x_odd  {1} else {2};
    println!("x = {x}");

    let mut y = 0;

let measly = loop {

    if y == 12345 {
        break y * 10;
    }
    y += 1;
    println!("x is {y}");
};
println!("Loop has ended and measly {measly}");

let mut poop = 0;

while poop < 97 {
    poop += 1;
    println!("Poop value is {poop}");
}

let mut arraycount = 0;
let dumble = ['a','b','c','d','e'];

while arraycount < dumble.len() {
    println!("dumble is {}", dumble[arraycount]);
    arraycount += 1;
}

for (index, &item) in dumble.iter().enumerate() {
    println!("Using for the item is {item} with index {index}");
}


for range_index in 0..254 {
println!("The range index is {range_index}");
}

let mut matrix = [[1,2,3],
                             [4,5,6],
                             [7,8,9]];

for row in matrix {
    for mut num in row {
        num += 10;
        print!("{num}\t");
    }
    println!();
}

let numbers = [1, 9,-2, 0, 23,20, -7, 13,37, 20, 56, -18, 20,3];
let mut max: i32;
let mut min: i32;
let mut mean: f64;

max = 0;
min = 0;

let mut temptotal = 0;

for item in numbers {
    temptotal += item;
    if item > max {max = item};
    if item < min {min  = item};
}

mean = temptotal as f64 / numbers.len() as f64;

assert_eq!(max, 56);
assert_eq!(min, -18);
assert_eq!(mean, 12.5);
println!("Tests passed!");

}


