// User 구조체 선언
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // User 구조체의 user1 인스턴스 생성
    // let user1 = User {
    //     active: true,
    //     username: String::from("someusername123"),
    //     email: String::from("someone@example.com"),
    //     sign_in_count: 1,
    // };

    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anothermail@example.com");
}
