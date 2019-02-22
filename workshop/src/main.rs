#![allow(dead_code)]

fn println_example() {
    println!("Hello, world!");
}

fn variables_example() {
    let name = "ashley";
    let age = 30;
    println!("Hi, {}! You are {} years old.", name, age);
}

fn mutability_example() {
    let mut apples = 100;
    apples += 50;
    println!("I have {} apples", apples);
}

fn add_fifty(n: i32) -> i32 {
    n + 50
}

fn if_example() {
    let height = 167u32;
    if height < 150 {
        println!("You're too small to go on the rollercoaster.");
    } else if height < 200 {
        println!("You may go on the rollercoaster!");
    } else {
        println!("You're too tall to go on the rollercoaster.");
    }
}

fn match_example() {
    let height = 167u32;
    match height {
        0..=150 => println!("You're too small to go on the rollercoaster."),
        150...200 => println!("You may go on the rollercoaster!"),
        _ => {
            println!("You're too tall to go on the rollercoaster.");
        },
    }
}

fn array_example() {
    let mut color = [255, 0, 255];
    // let mut color [i32; 3] = [255, 0, 255]; // Another way that we could have defined this
    color[0] = 100;
    println!("The color is {:?}", color);
}

fn vectors_example() {
    let mut prices = vec![30, 100, 2];
    //let mut prices:Vec<i32> = vec![30, 100, 2]; // Another way that we could have defined this
    prices[0] = 25;
    prices.push(40);
    println!("All the prices are: {:?}", prices);
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

fn formatting_example() {
    // Standard type
    let a = 150;

    // Custom type
    let p = Person{
        name: String::from("Arjan"), 
        age: 41
    };

    println!("Display");
    println!("a: {}", a);
    //println!("p: {}", p); // Doesn't work. Find out why: https://doc.rust-lang.org/std/fmt/trait.Display.html
    println!();
    
    println!("Debug");
    println!("a: {:?}", a);
    println!("p: {:?}", p);
    println!();

    println!("Pretty debug");
    println!("a: {:#?}", a);
    println!("p: {:#?}", p);
}

fn for_loop_example() {
    // exclusive range
    for i in 0..10 {
        println!("Number {}", i);
    }

    // inclusive range
    for i in 0..=10 { // In older code ... is still used
        println!("Number {}", i);
    }

    // Looping over vectors is done via an iterator. 
    // A good article about iterators is: https://hermanradtke.com/2015/06/22/effectively-using-iterators-in-rust.html
    let names = vec!["Carol", "Jake", "Marylou", "Bruce"];
    for name in names.iter() {
        println!("Hi {}!", name);
    }
}

fn while_loop_example() {
    let mut i = 10;
    while i > 0 {
        println!("Number {}", i);
        i -= 1;
    }
}

fn loop_example() {
    let mut i = 10;
    loop {
        println!("Number {}", i);
        if i == 0 {
            break
        }
        i -= 1;
    }
}

fn filter_example() {
    for i in (0..10).filter(|x| x % 2 == 0) {
        println!("i = {}", i);
    }
}

fn map_example() {
    for i in (0..10).map(|x| x * x ) {
        println!("i = {}", i);
    }
}

fn sum_example() {
    let sum = (0..10).fold(0, |acc, x| acc + x);
    println!("sum = {}", sum);
}

#[derive(PartialEq, Debug)] // Needed for testing
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn enum_example() {
    let light = TrafficLight::Green;
    match light {
        TrafficLight::Red => println!("STOP!"),
        TrafficLight::Yellow => println!("Slow down!"),
        TrafficLight::Green => println!("Go go go!"),
    }
}

enum GameType {
    SinglePlayer,
    MultiPlayer(u32),
}

fn enum_with_value_example() {
    let game = GameType::MultiPlayer(4);
    match game {
        GameType::SinglePlayer => println!("How about solitaire?"),
        GameType::MultiPlayer(2) => println!("How about checkers?"),
        GameType::MultiPlayer(4) => println!("How about bridge?"),
        GameType::MultiPlayer(num) => {
            println!("How about {}-player tag?", num)
        },
    }
}

fn option_example() {
    let mut instructors = vec!["Carol"];
    let a = instructors.pop();
    println!("a is {:?}", a);
    let b = instructors.pop();
    println!("b is {:?}", b);
}

fn use_option_example() {
    let a = Some("Carol");
    let name = a.expect("No name present");
    println!("Name is {} bytes long", name.len());

    let b: Option<&str> = None;
    match b {
        Some(name) => {
            println!("Other name is {} bytes long", name.len())
        },
        None => {
            println!("No name!")
        }
    }
}

fn result_example() {
    let numstr = "6";
    let num = numstr.parse::<i32>();
    println!("num = {:?}", num);

    let numstr = "florp";
    let num = numstr.parse::<i32>();
    println!("num = {:?}", num);
}

fn add_five_to_string(s: String) ->
    Result<i32, std::num::ParseIntError> {
    let ans = s.parse::<i32>()? + 5;
    Ok(ans)
}

fn slice_example() {
    let v = vec![1, 2, 3, 4, 5];
    let piece = &v[3..];
    println!("piece of v = {:?}", piece);

    let s = String::from("Call me Ishmael blah blah...");
    let part = &s[0..4];
    println!("part is '{}'", part);
}

fn main() {
    // println_example();

    // variables_example();
    
    // mutability_example();
    
    // println!("100 + 50 = {}", add_fifty(100));

    // if_example();

    // match_example();

    // array_example();

    // vectors_example();

    // formatting_example();

    // for_loop_example();

    // while_loop_example();

    // loop_example();

    // filter_example();

    // map_example();

    // sum_example();

    // enum_example();

    // enum_with_value_example();

    // option_example();

    // use_option_example();

    // result_example();

    // match add_five_to_string("10".to_string()) {
    //     Ok(res) => println!("Results: {}", res),
    //     Err(e) => println!("Error: {:?}", e),
    // };

    // slice_example();
}

#[cfg(test)]
mod tests {
    use super::TrafficLight;

   #[test]
    fn traffic_light_works() {
        let light = TrafficLight::Yellow;

        assert!(light == TrafficLight::Yellow);
        assert_eq!(light, TrafficLight::Yellow);
        assert_ne!(light, TrafficLight::Red);
    }

    #[test]
    #[ignore]
    fn not_implemented_yet() {
        assert_eq!(true, false);
    }
}
