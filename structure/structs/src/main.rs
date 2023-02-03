struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// fn build_suer (email:String,username:String) -> User{
//     User {
//         active:true,
//         username,
//         email,
//         sign_in_count:1,
//     }
// }

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}
