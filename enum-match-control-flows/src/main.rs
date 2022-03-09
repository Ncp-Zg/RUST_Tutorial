enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

mod back_of_house;

pub use crate::back_of_house::Breakfast;

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    // meal.toast = String::from("Wheat");
    println!("I'd like {:?} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny =>1,
        Coin::Nickel =>5,
        Coin::Dime =>10,
        Coin::Quarter(state) =>{
            println!("State quarter from {:?}",state);
            25
        },

    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}

fn main() {

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    // let config_max = Some(3u8);
    // match config_max {
    //     Some(max) => println!("The maximum is configured to be {}", max),
    //     _ => (),
    // }

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    println!("{}",value_in_cents(Coin:: Quarter(UsState::Alabama)));
    println!("{:?} {:?} {:?}",five,six,none);
    println!("{:?}",config_max);
    println!("{:?}",dice_roll);
    println!("{:?}",eat_at_restaurant());
}


fn add_fancy_hat() {
    println!("add")
}

fn remove_fancy_hat() {
    println!("remove")
}