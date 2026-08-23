fn main() {
    let excerpt = String::from(
        "It was the best of times, it was the worst of times, it was the age of wisdom, it was the age of foolishness, it was the epoch of belief, it was the epoch of incredulity, it was the season of Light, it was the season of Darkness, it was the spring of hope, it was the winter of despair, we had everything before us, we had nothing before us, we were all going direct to Heaven, we were all going direct the other way-in short, the period was so far like the present period, that some of its noisiest authorities insisted on its being received, for good or for evil, in the superlative degree of comparison only…",
    );

    println!("The length is {}", get_length(&excerpt));
    println!("The characters count is {}", get_characters_count(&excerpt));
    println!("The words count is {}", get_words_count(&excerpt));
    println!("The Longest words is {}", get_longest_word(&excerpt));
}


fn get_length(message:&str)->usize{
    message.len()
}

fn get_characters_count(message: &str)->usize{
     message.chars().count()
}

fn get_words_count(message: &str)->usize{
    message.split_whitespace().count()
}

fn get_longest_word(message: &str)-> &str{
   let max_word= message.split_whitespace().max_by_key(|word| word.len()).unwrap_or("");
    max_word
}
