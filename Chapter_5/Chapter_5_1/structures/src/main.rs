
// remind to user CamelCase to define structures
struct User{

    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
} // No semi-colon here

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-like struct

struct AlwaysEqual;


fn main() {
    

    let mut user1 = User{
        email : String::from("someone@example.com"),
        username : String::from("someusername123"),
        sign_in_count: 1,
        active: true
    };

    // modify struct values

    user1.email  = String::from("anothersomeone@example.com"); // Such modification make user1 no longer available to scotruct another user

    let user2 = User{
        active: user1.active,
        username: user1.username,
        email: String::from("anotheremail@example.com"),
        sign_in_count: user1.sign_in_count
    };

    let mut user3 = User {
        email: String::from("another@example.com"),
        ..user2
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;
}


fn build_user(email: String, username: String) -> User{

    User{email,
        username, 
        active: true,
        sign_in_count: 1}
}