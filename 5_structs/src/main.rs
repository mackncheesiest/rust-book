struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct TupleStruct(i32, i32, i32);

struct UnitStruct;

fn main() {
    let user1 = User {
        username: String::from("username"),
        email: String::from("username@email.com"),
        sign_in_count: 1,
        active: true
    };

    let user2 = construct_user(String::from("username2"), 
                               String::from("username2@email.com"));

    let user3 = User {
        username: String::from("user3"),
        email: String::from("user3@email.com"),
        ..user2
    };

    print_user(&user1);
    print_user(&user2);
    print_user(&user3);

    let tuple = TupleStruct(3, 4, 5);
    println!("Tuple struct has field values: {}, {}, {}", tuple.0, tuple.1, tuple.2);

    let _nil = UnitStruct;
}

fn construct_user(username: String, email: String) -> User {
    User {
        username: username,
        email: email,
        sign_in_count: 1,
        active: true
    }
}

fn print_user(user: &User) {
    println!("Values: {}, {}, {}, {}", user.username, user.email, user.sign_in_count, user.active);
}
