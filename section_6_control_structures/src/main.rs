use rand::Rng;
use crate::Suit::{Heart, Club, Diamond, Spade};

#[allow(unused_variables)]
#[allow(unused_assignments)]

fn main() {
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(0, 11);

    // =============== IF/ELSE ===============

    // if num >= 5 {
    //     println!("Number {} is greater than or equal to 5", num);
    // }
    // else{
    //     println!("Number {} is smaller than 5", num);
    // }

    if num > 5{
        println!("{} > 5", num);
    }
    else if num == 5{
        println!("{} == 5", num);
    }
    else{
        println!("{} < 5", num);
    }

    let res = if num >= 5 {true} else {false};
    println!("{}", res);
    println!("");

    // =============== MATCH ===============
    print_choice(Heart);
    print_choice(Club);
    print_choice(Diamond);
    print_choice(Spade);

    country(44);
    country(34);
    country(125);
    country(-15);
    println!("");

    // =============== MATCH ===============
    for i in 0..=15{
        println!("{}. I have {} oranges", i, get_oranges(i));
    }
    println!("");

    // Tuple matching
    let point = (10, 6);

    match point{
        (0, 0) => println!("origin"),
        (x, 0) => println!("x axis ({}, 0)", x),
        (0, y) => println!("y axis (0, {})", y),
        (x, y) => println!("({}, {})", x, y)
    }
}

fn get_oranges(amount: i32) -> &'static str {
    return match amount{
        0 => "no",
        1 | 2 => "one or two",
        3..=7 => "a few",
        _ if (amount % 2 == 0) => "an even amount of",
        _ => "lots of"
    }
}

fn country(code: i32){
    let country = match code{
        44 => "UK",
        34 => "Spain",
        1..=99 => "Unknown",
        _ => "Invalid"
    };
    println!("Country is {}", country);
}

enum Suit {
    Heart,
    Spade,
    Club,
    Diamond
}

fn print_choice(choice: Suit){
    match choice{
        Heart => {println!("\u{2665}")}
        Spade => {println!("\u{2660}")}
        Club => {println!("\u{2663}")}
        Diamond => {println!("\u{2666}")}
    }
}
