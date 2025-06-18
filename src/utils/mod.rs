pub  struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

pub struct Rect{
    width: u32,
    height: u32,
}

impl Rect{
    pub fn area(&self) -> u32 {
        self.width * self.height
    }
}

pub fn test_rectangle(){
    let rectangle = Rect{
        width: 30,
        height: 50,
    };

    println!("Area of reactangle is {}", rectangle.area());
}

pub fn create_user(){
    let user1 = User{
        active: true,
        username: String::from("Test1"),
        email: String::from("test1@gmail.com"),
        sign_in_count: 1,
    };

    println!("User1 name {:?}", user1.username);
}