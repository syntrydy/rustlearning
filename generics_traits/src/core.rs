use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};

pub trait Accommodation {
    fn get_description(&self) -> String;

    fn get_info(&self) -> String {
        String::from("A beautifull place to stay!")
    }
}

pub trait Bookable {
    fn book(&mut self, name: &str, nights: u32);
}

#[derive(Debug)]
pub struct Hotel {
    name: String,
    reservations: HashMap<String, u32>,
}
#[derive(Debug)]
pub struct AirBnb {
    host: String,
    guests: Vec<(String, u32)>,
}

impl Hotel {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            reservations: HashMap::new(),
        }
    }
}

impl Accommodation for Hotel {
    fn get_description(&self) -> String {
        format!("{} is your current accommodation", self.name.to_uppercase())
    }

    fn get_info(&self) -> String {
        self.name.to_string()
    }
}

impl Bookable for Hotel {
    fn book(&mut self, name: &str, nights: u32) {
        self.reservations.insert(name.to_string(), nights);
    }
}

impl AirBnb {
    pub fn new(host: &str) -> Self {
        Self {
            host: host.to_string(),
            guests: Vec::new(),
        }
    }

    pub fn pretty_print(&self) {
        println!("***************************");
        println!("{}", self.get_description().to_uppercase());
        println!("***************************")
    }
}

impl Accommodation for AirBnb {
    fn get_description(&self) -> String {
        format!(
            "Enjoy your appartment stay. Provides with joy by {}",
            self.host
        )
    }

    fn get_info(&self) -> String {
        self.host.to_string()
    }
}

impl Bookable for AirBnb {
    fn book(&mut self, name: &str, nights: u32) {
        self.guests.push((name.to_string(), nights));
    }
}

pub fn book_for_a_week(acco: &mut (impl Accommodation + Bookable), guest_name: &str) {
    acco.book(guest_name, 7);
}

pub fn book_for_ten_days<T: Accommodation + Bookable>(acco: &mut T, guest_name: &str) {
    acco.book(guest_name, 7);
}

pub fn choose_the_bestplace_tostay() -> impl Accommodation {
    return Hotel::new("Cristal");
}

pub struct Person {
    pub name: String,
    pub status: UserState,
    pub age: u8,
}

impl Display for Person {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        write!(
            formatter,
            "This is {} of {} years old, current status {}",
            self.name, self.age, self.status
        )
    }
}

pub enum UserState {
    LogIn,
    Logout,
}

impl Display for UserState {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            UserState::LogIn => write!(f, "**Login**"),
            UserState::Logout => write!(f, "**Logout**"),
        }
    }
}
