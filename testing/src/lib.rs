pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub struct Guess {
    value: i32,
}
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1{
            panic!(
                "Guess value must be greater than 0, got {value}."
            );
        }
        else if value > 100 {
            panic!(
                "Guess value must be less than 100, got {value}."
            );
        }
        Guess {value}
   }
}

#[cfg(test)]
mod testing {

use super::*;

    #[test]
    fn exploration() -> Result<(),String> {
        let result = add(2, 2);
        if result == 4 {
            return Ok(())
        }
        Err(String::from("two plus two does not equal four"))
    }

    #[test]
    #[should_panic(expected="less than 100")]
    fn new_guess_overflow(){
       Guess::new(200);
    }


    #[test]
    #[should_panic(expected="greater than 0")]
    fn new_guess_underflow(){
       Guess::new(0);
    }
}
