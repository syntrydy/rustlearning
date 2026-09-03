pub mod core;

use crate::core::*;

fn main() {
    let mut madison = Hotel::new(&"Madison Hotel".to_string());
    println!("{:#?}", madison);
    println!("{}", madison.get_info());
    println!("{}", madison.get_description());
    madison.book("Gasmyr", 3);
    println!("{:#?}", madison);

    let mut residence = AirBnb::new("Residence la Colombe");
    println!("{:#?}", residence);
    println!("{}", residence.get_info());
    println!("{}", residence.get_description());
    residence.book("Jeseay", 5);
    println!("{:#?}", residence);
    residence.pretty_print();

    book_for_a_week(&mut madison, &"Awa".to_string());
    println!("{:#?}", madison);
    book_for_a_week(&mut residence, &"Awa".to_string());
    book_for_ten_days(&mut residence, &"Julie".to_string());
    println!("{:#?}", residence);
    println!(
        "The best place to stay is: {:?}",
        choose_the_bestplace_tostay().get_info()
    );

    let facilities: Vec<&dyn Accommodation> = vec![&residence, &madison];
    for facility in facilities {
        facility.get_description();
    }

    let thomy = Person {
        name: "Thomy".to_string(),
        age: 24,
        status: UserState::LogIn,
    };

    println!("{}", thomy);
}
