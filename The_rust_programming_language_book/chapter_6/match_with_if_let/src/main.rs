fn main() {
    // config max using match
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maxium is configured to be {}", max),
        _ => (),
    }

    // config max using if let
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    // if let example function
    let mut count = 0;
    coin_counter(Coin::Dime, &mut count);
    println!("current coin count is {}", count); // else branch of coin_counter
    coin_counter(Coin::Quarter(UsState::Delaware), &mut count);
    println!("current coin count is {}", count); // if branch of coin_counter
}

// example of if let with else
fn coin_counter(coin: Coin, count: &mut i32) {
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
        *count += 25;
    } else {
        *count += 100;
    }
}

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
