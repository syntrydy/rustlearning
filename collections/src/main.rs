use std::collections::HashMap;

fn main() {

    let mut _name =String::from("My name is Mougang Thomas Gasmyr");

    let mut months: HashMap<u8, String> = HashMap::new();
    months.insert(1, "January".to_string());
    months.insert(2, "February".to_string());
    months.insert(3, "March".to_string());
    months.insert(4, "April".to_string());
    months.insert(5, "May".to_string());
    months.insert(6, "June".to_string());

    println!("{:?}", months.values());

    println!("{}",_name.replace("Thomas", "T."));
    
    let name = "Dag Wirén";
    let result =name.get(..9);
    if let Some(value) = result{
        println!("{}", value);
    }
   
}


