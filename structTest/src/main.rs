
struct MyUser {
    username: String,
    email: String,
    sign_in_coungt: u64,
    active: bool,
}

// tuple struct

struct Color(i32, i32, i32);
fn main() {
    let mut user1 = MyUser {
        username: String::from("LDG"),
        email: String::from("lee032677@naver.com"),
        sign_in_coungt: 111,
        active: true,
    };

    // update username!!
    user1.username = String::from("lywoo");

    println!("username : {}", user1.username);
    println!("email : {}", user1.email);
    println!("sign_in_coungt : {}", user1.sign_in_coungt);
    println!("active : {}", user1.active);

    let mut user2 = build_user(String::from("testEmail"), String::from("Testusername"));
    println!("username2 : {}", user2.username);
    println!("email2 : {}", user2.email);
    println!("sign_in_coungt2 : {}", user2.sign_in_coungt);
    println!("active2 : {}", user2.active);



    let user3 = MyUser {
        // email: user2.email,
        // username: String::from("test3 username"),
        // active: true,
        // sign_in_coungt: 12,
        ..user1
    };
    println!("username3 : {}", user3.username);
    println!("email3 : {}", user3.email);
    println!("sign_in_coungt3 : {}", user3.sign_in_coungt);
    println!("active3 : {}", user3.active);

    let black = Color(1,2,3);

    println!("red : {}", black.0);
    println!("green : {}", black.1);
    println!("blue : {}", black.2);
}

fn build_user(email: String, username: String) -> MyUser {
    MyUser {
        email,
        username,
        active: true,
        sign_in_coungt: 10,
    }
}