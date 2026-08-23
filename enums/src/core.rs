#[derive(Debug)]
pub enum Colors {
    RED,
    GREEN,
    BLUE,
}
#[derive(Debug)]
pub enum Status {
    Alive,
    Dead(u32),
}
#[derive(Debug)]
pub enum Ordering {
    Less,
    Equal,
    Greater,
}

#[derive(Debug)]
pub enum NetworkMessage {
    Ping,
    Post(String),
    Quit { _id: String, reason: Option<String> },
}

#[derive(Debug)]
pub enum HttpStatus {
    Ok = 200,
    NotFound = 404,
    UnAuthenticated,
}

#[derive(Debug)]
pub enum TrafficLight{
    RED,
    YELLOW,
    GREEN
}

impl TrafficLight{

    pub fn can_go(&self) -> bool{
       match self {
           Self::GREEN => true,
           _=> false
       }
    }
}

pub fn compare(a: u32, b: u32) -> Ordering {
    if a > b {
        Ordering::Greater
    } else if a == b {
        Ordering::Equal
    } else {
        Ordering::Less
    }
}

pub fn process_message(n: NetworkMessage) {
    match n {
        NetworkMessage::Ping => {
            println!("Ping!!!!")
        }
        NetworkMessage::Post(message) => {
            println!("Message receive is {message}")
        }
        NetworkMessage::Quit { _id: _, reason } => {
            if reason.is_some(){
                 println!("I'm quitting for {}", reason.unwrap())
            }else {
                println!("I'm quitting now.")
            }
            
        }
    }
}

pub fn check_ordering(){
    match compare(25, 60) {
        Ordering::Less => {
            println!("Is less")
        }
        Ordering::Equal => {
            println!("Is equals");
        }
        Ordering::Greater => {
            println!("Is greater");
        }
    }
}
