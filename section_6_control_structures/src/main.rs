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
