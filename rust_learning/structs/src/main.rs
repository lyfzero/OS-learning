struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("lihua"),
        active: true,
        sign_in_count: 1,
    };
    let mut user2 = User {
        email: String::from("someone2@example.com"),
        username: String::from("han"),
        active: true,
        sign_in_count: 2,
    };
    user2.email = String::from("anotheremail@example.com");
    
    // 从其他实例创建
    let user3 = User {
        email: String::from("asd@example.com");
        username: String::from("liu");
        ..user1
    }
}

fn build_user(email: String, username: String) -> User { // 初始化简写
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

