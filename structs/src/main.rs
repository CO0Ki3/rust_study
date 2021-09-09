struct User {
    username: String,
    email: String,
    sign_in_count: i32,
    active: bool,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let mut user1 = User {
        username: String::from("독고현"),
        email: String::from("ehrrhgus0324@gmail.com"),
        sign_in_count: 1,
        active: true,
    };

    user1.email = String::from("co0ki3@transverse.ai");

    build_user(user1.email, String::from("co0ki3"));

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };
    println!("{}, {}, {}", user2.active, user2.username, user1.sign_in_count);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{}, {}", black.1, origin.2);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        sign_in_count: 1,
        active: true
    }
}