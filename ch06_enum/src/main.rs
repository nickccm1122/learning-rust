#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

impl Coin {
    fn value_in_coin(coin: Coin) -> u32 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
}

fn main() {
    println!("Value of a Quarter is {}", Coin::value_in_coin(Coin::Quarter));

    let five = 5;

    println!("this should print 5 + 1, {:?}", plus_one(Some(five)));

    some_u8_value(3);

    some_coin(Coin::Nickel);
    some_coin(Coin::Penny);
}


fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None
    }
}

// using `match`
fn some_u8_value(x: u8) {
    match x {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("I don't care")
    }
}

// using `if let`
fn some_coin(coin: Coin) {
    if let Coin::Dime = coin {
        println!("it is a dime!");
    } else if let Coin::Nickel = coin {
        println!("it is a nickel!");
    } else {
        println!("i don't care anymore");
    };
}
