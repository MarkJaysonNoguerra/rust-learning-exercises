enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

enum CoinV3 {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
    Connecticut,
    Delaware,
    Florida,
    Georgia,
    Hawaii,
    Idaho,
    Illinois,
    Indiana,
    Iowa,
    Kansas,
    KentuckyB,
    Louisiana,
    Maine,
    Maryland,
    MassachusettsB,
    Michigan,
    Minnesota,
    Mississippi,
    Missouri,
    Montana,
    Nebraska,
    Nevada,
    NewHampshire,
    NewJersey,
    NewMexico,
    NewYork,
    NorthCarolina,
    NorthDakota,
    Ohio,
    Oklahoma,
    Oregon,
    PennsylvaniaB,
    RhodeIsland,
    SouthCarolina,
    SouthDakota,
    Tennessee,
    Texas,
    Utah,
    Vermont,
    VirginiaB,
    Washington,
    WestVirginia,
    Wisconsin,
    Wyoming,
}
fn main() {
    println!("The value of Coin::Dime is {}", value_in_cents(Coin::Dime));

    println!(
        "The value of Coin::Penny is {}",
        value_in_cents_v2(Coin::Penny)
    );

    println!(
        "The value of Coin::Quarter is {}",
        value_in_cents_v3(CoinV3::Quarter(UsState::Arizona))
    );
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn value_in_cents_v2(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn value_in_cents_v3(coin: CoinV3) -> u8 {
    match coin {
        CoinV3::Penny => 1,
        CoinV3::Nickel => 5,
        CoinV3::Dime => 10,
        CoinV3::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
