struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Note that the entire instance must be mutable;
// Rust doesn’t allow us to mark only certain fields as mutable. ]
// As with any expression, we can construct a new instance of the struct as the last expression
// in the function body to implicitly return that new instance.
// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true,
//         username: username,
//         email: email,
//         sign_in_count: 1,
//     }
// }


// SHORT
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}


fn main() {
    // let user1 = User {
    //     active: true,
    //     username: String::from("my_user"),
    //     email: String::from("email@e.com"),
    //     sign_in_count: 1,
    // };

    let mut user1 = User {
        active: true,
        username: String::from("my_user"),
        email: String::from("email@e.com"),
        sign_in_count: 1,
    };

    // Change value
    user1.email = String::from("email@email.com");

    println!("{}", user1.email);

    let user2 = build_user(String::from("myemail2@email.com"), String::from("my_user2"),);

    println!("{}", user2.email);

    // Creating Instances from Other Instances with Struct Update Syntax

    let user3 = User{
        email: String::from("newemail.com"),
        ..user1
    };

    println!("{} - {}", user3.email, user3.username);

    // TUPLES
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
