
fn main() {
    let number: u32 = match std::env::args().nth(1) {
        Some(arg) => match arg.parse() {
            Ok(x) => x,
            Err(_) => {
                eprintln!("Should be an integer!");
                return;
            }
        },
        None => {
            eprintln!("Missing arguments");
            return;
        }
    };

    println!("{}", number);
    match number {
        x if x % 2 ==0 =>{
            println!("{x} is even");
        }
        _=>{
            println!("{number} is odd");
        }
    }
}
