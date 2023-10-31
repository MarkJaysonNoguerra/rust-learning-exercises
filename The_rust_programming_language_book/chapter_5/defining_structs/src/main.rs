fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    println!("The user1 email is {}", user1.email);

    println!(
        "The build user username is {}",
        build_user(String::from("email@.com"), String::from("test")).username
    );

    println!(
        "The build user shorthand username is {}",
        build_user(String::from("email@.com"), String::from("test_shorthand")).username
    );

    // building another struct instance
    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };

    // println!("user1 email is {}", user1.email);
    // println!("user2 email is {}", user2.email);

    // building another struct instance shorthand
    let user2 = User {
        email: String::from("another_another@example.com"),
        ..user1
    };

    println!("user1 email is {}", user1.email);
    println!("user2 email is {}", user2.email);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

fn build_user_shorthand(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
