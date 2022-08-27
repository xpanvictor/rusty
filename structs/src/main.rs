
struct User {
    name: String,
    age: i32,
    email: String,
    is_active: bool
}

fn main() {
    let user1 = User {
        name: String::from("Samson"),
        age: 12,
        email: String::from("sam@lok.com"),
        is_active: false
    };

    println!("First user's name is {} with age {}", user1.name, user1.age);
    let mut user2 = build_user(String::from("Skay"), String::from("skay@gmail.com"));
    println!("First user's name is {} with email {} and is{}active", user2.name, user2.email, if user2.is_active {""} else {" not "});
    // make user2 active
    user2.is_active = true;
    println!("is second user {} active?  {}", user2.name, if user2.is_active {"yes"} else {"no"});
    // copy user2's data n make new user
    let user3 = User {
        name: String::from("Simpson"),
        age: 15,
        ..user2
    };
    // borrow of moved value: `user2.email`
    // move occurs because `user2.email` has type `String`, which does not implement the `Copy` trait
    // hence we cant use user2 email again
    println!("First user's name is {} with email {}", user3.name, user3.is_active);
    println!("is second user {} active?  {}", user2.name, if user2.is_active {"yes"} else {"no"});
}

fn build_user (name: String, email: String) -> User {
    User{
        name,
        email,
        age: 0,
        is_active: false
    }
}