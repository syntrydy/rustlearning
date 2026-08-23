#[derive(Debug)]
pub struct Person {
    first_name: String,
    last_name: String,
    age: u8,
    active: bool,
}

#[derive(Debug)]
pub enum Colors{
    RED,
    GREEN,
    BLUE
}

impl Colors{
    pub fn show(&self){
        println!("{:?}", self)
    }
}

impl Person {
    pub fn new(first_name: String, last_name: String, age: u8) -> Self {
        Self {
            first_name,
            last_name,
            age,
            active: false,
        }
    }
}
