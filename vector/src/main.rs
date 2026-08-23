
#[derive(Debug)]
enum Colors{
    RED,GREEN,BLUE
}
#[derive(Debug)]
enum Status{
    Alive,
    Dead(u32)
}
#[derive(Debug)]
enum Ordering {
      Less,
      Equal,
      Greater
}

fn  compare(a:u32,b:u32)-> Ordering{
   if a > b{
      Ordering::Greater
   }else if a == b  {
       Ordering::Equal
   }else{
    Ordering::Less
   }
}

fn main() {
   let color= Colors::BLUE;
    println!(" {:#?}", color);
    let dead = Status::Dead(2000);
    println!(" {:?}", dead);
    match compare(25, 60) {
        Ordering::Less => {println!("Is less")}
        Ordering::Equal => {println!("Is equals");}
        Ordering::Greater => {println!("Is greater");}
    }
}